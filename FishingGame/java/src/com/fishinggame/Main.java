package com.fishinggame;

import javax.swing.*;
import java.awt.*;
import java.awt.event.WindowAdapter;
import java.awt.event.WindowEvent;
import java.awt.geom.AffineTransform;
import java.awt.image.BufferedImage;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.HashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Random;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
    private static final Path SAVE_PATH = Path.of("save", "progress.json");
    private static final int MAX_STAMINA = 100;
    private static final int MAX_ROD_LEVEL = 8;

    private int fishCaught = 0;
    private int totalCatchValue = 0;
    private int gold = 0;
    private int lifetimeGoldEarned = 0;
    private int stamina = MAX_STAMINA;
    private int rodLevel = 1;
    private int baitLevel = 1;
    private String currentLocation = "Pond";
    private String weather = "Clear";
    private String moonPhase = "Normal";
    private int castsSinceWeatherUpdate = 0;
    private int luckBuffCasts = 0;
    private int valueBuffCasts = 0;
    private int specialRodPower = 0;
    private boolean soundsEnabled = true;
    private int relationMarina = 0;
    private int relationBrokerFinn = 0;
    private int relationTinkerKai = 0;
    private String playerCharacter = PlayerArchetype.ALEX.displayName;

    private final Random random = new Random();
    private final Deque<Integer> lastTenValues = new ArrayDeque<>();
    private final List<FishResult> inventory = new ArrayList<>();
    private FishResult leastValueFish;
    private FishResult mostValueFish;
    private TradeOffer tradeOffer;

    private JLabel statusLabel;
    private JLabel statsLabel;
    private JLabel worldLabel;
    private JLabel rodSkinLabel;
    private JLabel rodSkinImageLabel;
    private JTextArea logArea;
    private JButton castButton;
    private JButton restButton;
    private JButton tradeButton;
    private final Map<Integer, ImageIcon> rodSkinIcons = new HashMap<>();
    private final Map<String, BufferedImage> imageCache = new HashMap<>();
    private final RestManager restManager = new RestManager();

    public static void main(String[] args) {
        SwingUtilities.invokeLater(() -> new Main().createAndShowUi());
    }

    private void createAndShowUi() {
        JFrame frame = new JFrame("Fishing Game");
        frame.setDefaultCloseOperation(JFrame.DO_NOTHING_ON_CLOSE);
        frame.setSize(980, 560);
        frame.setLocationRelativeTo(null);
        boolean hadSave = loadProgress();
        updateWorldCycle();
        ensureLocationAccess();

        JPanel root = new JPanel(new BorderLayout(10, 10));
        root.setBorder(BorderFactory.createEmptyBorder(12, 12, 12, 12));

        JLabel title = new JLabel("Fishing Game");
        title.setFont(new Font(Font.SANS_SERIF, Font.BOLD, 24));

        statusLabel = new JLabel("Ready to fish.");
        statsLabel = new JLabel(getStatsText());
        worldLabel = new JLabel(getWorldText());
        rodSkinLabel = new JLabel(getRodSkinText());

        rodSkinImageLabel = new JLabel();
        rodSkinImageLabel.setPreferredSize(new Dimension(180, 46));

        JPanel rodPanel = new JPanel(new FlowLayout(FlowLayout.LEFT, 8, 0));
        rodPanel.add(rodSkinImageLabel);
        rodPanel.add(rodSkinLabel);

        JPanel top = new JPanel(new GridLayout(5, 1, 4, 4));
        top.add(title);
        top.add(statusLabel);
        top.add(statsLabel);
        top.add(worldLabel);
        top.add(rodPanel);

        castButton = new JButton("Cast Line");
        castButton.addActionListener(e -> onCastLine(castButton));
        JButton shopButton = new JButton("Shop");
        shopButton.addActionListener(e -> openShop());
        JButton sellButton = new JButton("Sell Inventory");
        sellButton.addActionListener(e -> sellInventoryInShop());
        tradeButton = new JButton("Trade Offer");
        tradeButton.addActionListener(e -> tryTradeOffer());
        JButton newTradeButton = new JButton("New Trade Offer");
        newTradeButton.addActionListener(e -> generateTradeOffer());
        JButton locationButton = new JButton("Locations");
        locationButton.addActionListener(e -> chooseLocation());
        restButton = new JButton("Rest (+25 stamina)");
        restButton.addActionListener(e -> rest());
        JButton saveButton = new JButton("Save");
        saveButton.addActionListener(e -> saveProgressAndReport());

        logArea = new JTextArea();
        logArea.setEditable(false);
        logArea.setLineWrap(true);
        logArea.setWrapStyleWord(true);
        logArea.setText("Catch log:\n");
        logArea.append("- You can sell fish at shop or wait for a timed trade offer.\n");

        root.add(top, BorderLayout.NORTH);
        root.add(new JScrollPane(logArea), BorderLayout.CENTER);

        JPanel actionPanel = new JPanel(new FlowLayout(FlowLayout.RIGHT));
        actionPanel.add(shopButton);
        actionPanel.add(sellButton);
        actionPanel.add(tradeButton);
        actionPanel.add(newTradeButton);
        actionPanel.add(locationButton);
        actionPanel.add(restButton);
        actionPanel.add(saveButton);
        actionPanel.add(castButton);
        root.add(actionPanel, BorderLayout.SOUTH);

        frame.addWindowListener(new WindowAdapter() {
            @Override
            public void windowClosing(WindowEvent e) {
                handleExit(frame);
            }
        });
        Timer uiPulseTimer = new Timer(500, e -> refreshUi());
        uiPulseTimer.start();
        frame.addWindowListener(new WindowAdapter() {
            @Override
            public void windowClosed(WindowEvent e) {
                uiPulseTimer.stop();
            }
        });

        frame.setContentPane(root);
        frame.setVisible(true);
        refreshUi();
        if (!hadSave) {
            chooseStartingCharacter(frame);
            maybeShowNewPlayerGuide(frame);
        }
    }

    private void onCastLine(JButton castButton) {
        if (stamina < getStaminaCostPerCast()) {
            statusLabel.setText("Too tired. Rest or buy stamina potion.");
            return;
        }

        castButton.setEnabled(false);
        int castMs = getCastTimeMs();
        statusLabel.setText("Casting... (" + castMs + " ms)");
        stamina = Math.max(0, stamina - getStaminaCostPerCast());
        statsLabel.setText(getStatsText());

        SwingWorker<FishResult, Void> worker = new SwingWorker<>() {
            @Override
            protected FishResult doInBackground() {
                try {
                    Thread.sleep(castMs);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                }
                return runFishEngine();
            }

            @Override
            protected void done() {
                castButton.setEnabled(true);
                try {
                    FishResult fish = get();
                    if (fish == null || fish.error != null) {
                        String err = fish == null ? "Unknown engine error." : fish.error;
                        statusLabel.setText("Engine error");
                        logArea.append("- Failed to catch fish: " + err + "\n");
                        return;
                    }

                    boolean caught = playCatchingMiniGame(fish);
                    consumeBuffDurations();
                    castsSinceWeatherUpdate++;
                    if (castsSinceWeatherUpdate >= 4) {
                        updateWorldCycle();
                    }
                    if (caught) {
                        fishCaught++;
                        totalCatchValue += fish.value;
                        updateValueStats(fish);
                        inventory.add(fish);
                        statusLabel.setText("You caught a " + fish.rarity + " " + fish.species + " (stored).");
                        logArea.append(String.format("- %s | %.2fkg | %s | value %d (in inventory)%n",
                                fish.species, fish.weightKg, fish.rarity, fish.value));
                    } else {
                        statusLabel.setText("The fish escaped during the pull.");
                        logArea.append("- The fish escaped after a bad pull sequence.\n");
                    }
                    ensureLocationAccess();
                    refreshUi();
                    statsLabel.setText(getStatsText());
                } catch (Exception ex) {
                    statusLabel.setText("Engine error");
                    logArea.append("- Failed to process catch: " + ex.getMessage() + "\n");
                }
            }
        };

        worker.execute();
    }

    private String getStatsText() {
        PlayerArchetype archetype = PlayerArchetype.fromName(playerCharacter);
        if (fishCaught == 0) {
            return "<html>Gold: " + gold + " | Stamina: " + stamina + "/" + MAX_STAMINA
                    + " | Rod L" + rodLevel + " | Bait L" + baitLevel
                    + " | Inventory: " + inventory.size()
                    + " | " + archetype.displayName
                    + " | Special Mod: " + specialRodPower
                    + "<br/>Caught: 0 | Total Value: 0 | Avg: 0.00 | Avg Last 10: 0.00"
                    + "<br/>Least: N/A"
                    + "<br/>Most: N/A</html>";
        }

        double average = (double) totalCatchValue / fishCaught;
        double averageLastTen = getAverageLastTen();

        return "<html>"
                + "Gold: " + gold + " | Stamina: " + stamina + "/" + MAX_STAMINA
                + " | Rod L" + rodLevel + " | Bait L" + baitLevel
                + " | Inventory: " + inventory.size() + " | " + archetype.displayName + " | Special Mod: " + specialRodPower + "<br/>"
                + "Caught: " + fishCaught + " | Total Value: " + totalCatchValue + "<br/>"
                + String.format("Avg: %.2f | Avg Last 10: %.2f<br/>", average, averageLastTen)
                + "Least: " + fishToValueRarityText(leastValueFish) + "<br/>"
                + "Most: " + fishToValueRarityText(mostValueFish)
                + "</html>";
    }

    private String getRodSkinText() {
        return "Rod Skin: " + getRodSkinName()
                + " | Catch Assist: -" + String.format(Locale.US, "%.0f%%", getRodCatchDifficultyReduction() * 100.0)
                + " difficulty";
    }

    private String getRodSkinName() {
        return switch (rodLevel) {
            case 0 -> "Worn Reed Pole";
            case 1 -> "Bamboo Starter";
            case 2 -> "Fiberglass Trail";
            case 3 -> "Composite Stream";
            case 4 -> "Graphite Pro";
            case 5 -> "Carbon Elite";
            case 6 -> "Titan Weave";
            case 7 -> "Abyss Arc";
            default -> "Mythril Current";
        };
    }

    private double getRodCatchDifficultyReduction() {
        double base = switch (rodLevel) {
            case 1 -> 0.00;
            case 2 -> 0.08;
            case 3 -> 0.14;
            case 4 -> 0.20;
            case 5 -> 0.27;
            case 6 -> 0.33;
            case 7 -> 0.39;
            default -> 0.46;
        };
        return Math.min(0.65, base + specialRodPower * 0.03);
    }

    private int getRelationship(String trader) {
        return switch (trader) {
            case "Marina" -> relationMarina;
            case "Broker Finn" -> relationBrokerFinn;
            case "Tinker Kai" -> relationTinkerKai;
            default -> 0;
        };
    }

    private void addRelationship(String trader, int amount) {
        int adjusted = (int) Math.max(1, Math.round(amount * PlayerArchetype.fromName(playerCharacter).relationGainMultiplier));
        switch (trader) {
            case "Marina" -> relationMarina = Math.min(40, relationMarina + adjusted);
            case "Broker Finn" -> relationBrokerFinn = Math.min(40, relationBrokerFinn + adjusted);
            case "Tinker Kai" -> relationTinkerKai = Math.min(40, relationTinkerKai + adjusted);
            default -> {
            }
        }
    }

    private String getBestFriendTrader() {
        int marina = relationMarina;
        int finn = relationBrokerFinn;
        int kai = relationTinkerKai;
        if (marina >= finn && marina >= kai) {
            return "Marina";
        }
        if (finn >= marina && finn >= kai) {
            return "Broker Finn";
        }
        return "Tinker Kai";
    }

    private double getShopDiscountMultiplier() {
        int relation = getRelationship(getBestFriendTrader());
        double discount = Math.min(0.22, relation * 0.0075);
        return 1.0 - discount;
    }

    private int discountedCost(int baseCost) {
        double archetypeCostMult = PlayerArchetype.fromName(playerCharacter).shopCostMultiplier;
        return Math.max(1, (int) Math.round(baseCost * getShopDiscountMultiplier() * archetypeCostMult));
    }

    private void playUiSound(String type) {
        if (!soundsEnabled) {
            return;
        }
        if ("pull".equals(type)) {
            Toolkit.getDefaultToolkit().beep();
            return;
        }
        if ("error".equals(type)) {
            Toolkit.getDefaultToolkit().beep();
            Toolkit.getDefaultToolkit().beep();
            return;
        }
        Toolkit.getDefaultToolkit().beep();
    }

    private String getWorldText() {
        String tradeText = tradeOffer == null ? "No trade offer"
                : "Trade x" + String.format(Locale.US, "%.2f", tradeOffer.multiplier) + " for "
                + tradeOffer.minRarity + "+ by " + tradeOffer.traderName + " (" + tradeOffer.secondsRemaining() + "s left)";
        return "Location: " + currentLocation
                + " | Weather: " + weather
                + " | Moon: " + moonPhase
                + " | Rest CD: " + restManager.secondsRemaining() + "s"
                + " | Buffs(Luck/Value): " + luckBuffCasts + "/" + valueBuffCasts
                + " | Rel(M/F/K): " + relationMarina + "/" + relationBrokerFinn + "/" + relationTinkerKai
                + " | " + tradeText;
    }

    private int getStaminaCostPerCast() {
        int base = 8 - (rodLevel - 1);
        if (rodLevel >= 6) {
            base -= 1;
        }
        if ("Rain".equals(weather)) {
            base = Math.max(3, base - 2);
        }
        return Math.max(3, base);
    }

    private int getCastTimeMs() {
        PlayerArchetype archetype = PlayerArchetype.fromName(playerCharacter);
        int base = 1500 - (rodLevel - 1) * 170;
        if (rodLevel >= 6) {
            base -= 120;
        }
        if ("Rain".equals(weather)) {
            base -= 300;
        }
        base = (int) Math.round(base * archetype.castSpeedMultiplier);
        return Math.max(450, base);
    }

    private void updateValueStats(FishResult fish) {
        if (lastTenValues.size() == 10) {
            lastTenValues.removeFirst();
        }
        lastTenValues.addLast(fish.value);

        if (leastValueFish == null || fish.value < leastValueFish.value) {
            leastValueFish = fish;
        }
        if (mostValueFish == null || fish.value > mostValueFish.value) {
            mostValueFish = fish;
        }
    }

    private double getAverageLastTen() {
        if (lastTenValues.isEmpty()) {
            return 0.0;
        }

        int sum = 0;
        for (int value : lastTenValues) {
            sum += value;
        }
        return (double) sum / lastTenValues.size();
    }

    private String fishToValueRarityText(FishResult fish) {
        if (fish == null) {
            return "N/A";
        }
        return fish.value + " / " + fish.rarity + " (" + fish.species + ")";
    }

    private void refreshUi() {
        statsLabel.setText(getStatsText());
        worldLabel.setText(getWorldText());
        rodSkinLabel.setText(getRodSkinText());
        rodSkinImageLabel.setIcon(getRodSkinIconForLevel(rodLevel));
        tradeButton.setEnabled(tradeOffer != null && tradeOffer.isActive());
        castButton.setEnabled(stamina >= getStaminaCostPerCast());
        if (restButton != null) {
            long restSeconds = restManager.secondsRemaining();
            boolean canRest = restManager.canRestNow();
            restButton.setEnabled(canRest);
            restButton.setText(canRest ? "Rest (+25 stamina)" : "Rest (" + restSeconds + "s)");
        }
    }

    private boolean playCatchingMiniGame(FishResult fish) {
        JDialog dialog = new JDialog((Frame) null, "Pulling...", true);
        dialog.setDefaultCloseOperation(WindowConstants.DO_NOTHING_ON_CLOSE);
        dialog.setSize(900, 520);
        dialog.setLocationRelativeTo(null);
        dialog.setLayout(new BorderLayout(8, 8));

        int fishDifficulty = rarityRank(fish.rarity);
        double baseDifficulty = 0.9 + fishDifficulty * 0.35;
        double effectiveDifficulty = Math.max(0.75, baseDifficulty - getRodCatchDifficultyReduction());

        CatchBarPanel barPanel = new CatchBarPanel(effectiveDifficulty);
        barPanel.setPreferredSize(new Dimension(640, 120));
        CatchScenePanel scenePanel = new CatchScenePanel(
                getBufferedImage("assets/backgrounds/" + normalizeLocationForAsset(currentLocation) + ".png"),
                getBufferedImage("assets/rods/rod_level" + Math.max(1, Math.min(MAX_ROD_LEVEL, rodLevel)) + ".png")
        );
        scenePanel.setPreferredSize(new Dimension(860, 270));

        JLabel phaseLabel = new JLabel("Pull 1/3 to identify fish");
        JLabel helpLabel = new JLabel("Press [Pull Now] or SPACE when marker is in GREEN. YELLOW gives a chance, RED is risky.");
        JProgressBar catchProgressBar = new JProgressBar(0, 100);
        catchProgressBar.setValue(45);
        catchProgressBar.setStringPainted(true);
        catchProgressBar.setString("Catch Progress: 45%");
        JButton pullButton = new JButton("Pull Now");

        JPanel north = new JPanel(new GridLayout(2, 1));
        north.add(phaseLabel);
        north.add(helpLabel);
        dialog.add(north, BorderLayout.NORTH);
        JPanel centerPanel = new JPanel(new BorderLayout(4, 4));
        centerPanel.add(scenePanel, BorderLayout.CENTER);
        centerPanel.add(barPanel, BorderLayout.SOUTH);
        dialog.add(centerPanel, BorderLayout.CENTER);

        JPanel south = new JPanel(new BorderLayout(8, 8));
        south.add(catchProgressBar, BorderLayout.CENTER);
        south.add(pullButton, BorderLayout.EAST);
        dialog.add(south, BorderLayout.SOUTH);

        final int[] pullCount = {0};
        final int[] catchProgress = {45};
        final boolean[] finished = {false};
        final boolean[] caught = {false};

        final double[] markerPosition = {0.0};
        final double[] markerDirection = {1.0};
        final double markerSpeed = Math.min(0.030, 0.011 + effectiveDifficulty * 0.0038 + Math.max(0, rodLevel - 5) * 0.0015);

        Timer animationTimer = new Timer(16, e -> {
            markerPosition[0] += markerDirection[0] * markerSpeed;
            if (markerPosition[0] >= 1.0) {
                markerPosition[0] = 1.0;
                markerDirection[0] = -1.0;
            } else if (markerPosition[0] <= 0.0) {
                markerPosition[0] = 0.0;
                markerDirection[0] = 1.0;
            }
            barPanel.setMarkerPosition(markerPosition[0]);
            scenePanel.setMarkerPosition(markerPosition[0]);
            scenePanel.tick();
        });

        Runnable resolvePull = () -> {
            if (finished[0]) {
                return;
            }
            pullCount[0]++;
            scenePanel.playPullAnimation();
            playUiSound("pull");
            double zoneScore = getZoneScore(markerPosition[0], effectiveDifficulty);
            int delta = (int) Math.round(zoneScore * (17.0 - fishDifficulty * 1.2));
            catchProgress[0] = Math.max(0, Math.min(100, catchProgress[0] + delta));
            catchProgressBar.setValue(catchProgress[0]);
            catchProgressBar.setString("Catch Progress: " + catchProgress[0] + "%");

            if (pullCount[0] <= 3) {
                phaseLabel.setText("Pull " + pullCount[0] + "/3: identifying fish...");
            }
            if (pullCount[0] == 3) {
                statusLabel.setText("Fish identified: " + fish.rarity + " " + fish.species + ".");
                logArea.append("- Pull reveal: " + fish.rarity + " " + fish.species + "\n");
                phaseLabel.setText("Fish identified: " + fish.rarity + " " + fish.species);
            } else if (pullCount[0] > 3) {
                phaseLabel.setText("Reel battle: keep marker in green.");
            }

            if (catchProgress[0] >= 100) {
                finished[0] = true;
                caught[0] = true;
                playUiSound("ok");
                animationTimer.stop();
                dialog.dispose();
                return;
            }
            if (catchProgress[0] <= 0) {
                finished[0] = true;
                caught[0] = false;
                scenePanel.playFailAnimation();
                playUiSound("error");
                animationTimer.stop();
                dialog.dispose();
            }
        };

        pullButton.addActionListener(e -> resolvePull.run());
        InputMap inputMap = dialog.getRootPane().getInputMap(JComponent.WHEN_IN_FOCUSED_WINDOW);
        ActionMap actionMap = dialog.getRootPane().getActionMap();
        inputMap.put(KeyStroke.getKeyStroke("SPACE"), "pull_now");
        actionMap.put("pull_now", new AbstractAction() {
            @Override
            public void actionPerformed(java.awt.event.ActionEvent e) {
                resolvePull.run();
            }
        });

        animationTimer.start();
        dialog.setVisible(true);
        animationTimer.stop();
        return caught[0];
    }

    private double getZoneScore(double markerPos, double effectiveDifficulty) {
        double greenWidth = Math.max(0.14, 0.30 - effectiveDifficulty * 0.06);
        double yellowWidth = Math.max(0.16, 0.36 - effectiveDifficulty * 0.05);
        double greenStart = 0.5 - greenWidth / 2.0;
        double greenEnd = 0.5 + greenWidth / 2.0;
        double yellowStart = 0.5 - yellowWidth / 2.0;
        double yellowEnd = 0.5 + yellowWidth / 2.0;

        if (markerPos >= greenStart && markerPos <= greenEnd) {
            return 1.0;
        }
        if (markerPos >= yellowStart && markerPos <= yellowEnd) {
            return random.nextDouble() < 0.62 ? 0.4 : -0.2;
        }
        return -1.0;
    }

    private ImageIcon getRodSkinIconForLevel(int level) {
        int safeLevel = Math.max(1, Math.min(MAX_ROD_LEVEL, level));
        if (rodSkinIcons.containsKey(safeLevel)) {
            return rodSkinIcons.get(safeLevel);
        }

        BufferedImage source = getBufferedImage("assets/rods/rod_level" + safeLevel + ".png");
        if (source == null) {
            rodSkinIcons.put(safeLevel, null);
            return null;
        }
        Image scaled = source.getScaledInstance(168, 40, Image.SCALE_SMOOTH);
        ImageIcon icon = new ImageIcon(scaled);
        rodSkinIcons.put(safeLevel, icon);
        return icon;
    }

    private BufferedImage getBufferedImage(String relativePath) {
        if (imageCache.containsKey(relativePath)) {
            return imageCache.get(relativePath);
        }
        Path iconPath = Path.of(relativePath);
        if (!Files.exists(iconPath)) {
            imageCache.put(relativePath, null);
            return null;
        }
        try {
            BufferedImage source = javax.imageio.ImageIO.read(iconPath.toFile());
            imageCache.put(relativePath, source);
            return source;
        } catch (IOException e) {
            imageCache.put(relativePath, null);
            return null;
        }
    }

    private ImageIcon getScaledIcon(String relativePath, int width, int height) {
        BufferedImage source = getBufferedImage(relativePath);
        if (source == null) {
            return null;
        }
        return new ImageIcon(source.getScaledInstance(width, height, Image.SCALE_SMOOTH));
    }

    private void showTraderDialogue(TraderNpc npc, String title) {
        JLabel portrait = new JLabel(getScaledIcon(npc.portraitPath, 96, 96));
        JLabel line = new JLabel("<html><body style='width:220px'>" + npc.randomDialogue(random) + "</body></html>");
        JPanel panel = new JPanel(new BorderLayout(8, 8));
        panel.add(portrait, BorderLayout.WEST);
        panel.add(line, BorderLayout.CENTER);
        JOptionPane.showMessageDialog(null, panel, title + " - " + npc.displayName, JOptionPane.INFORMATION_MESSAGE);
    }

    private String normalizeLocationForAsset(String location) {
        if ("VolcanicBay".equals(location)) {
            return "volcanicbay";
        }
        return location.toLowerCase(Locale.ROOT);
    }

    private String getNextRodUnlockText() {
        if (rodLevel >= MAX_ROD_LEVEL) {
            return "Maxed (Mythril Current)";
        }
        int nextLevel = rodLevel + 1;
        String name = switch (nextLevel) {
            case 2 -> "Fiberglass Trail";
            case 3 -> "Composite Stream";
            case 4 -> "Graphite Pro";
            case 5 -> "Carbon Elite";
            case 6 -> "Titan Weave";
            case 7 -> "Abyss Arc";
            default -> "Mythril Current";
        };
        return "Rod L" + nextLevel + ": " + name;
    }

    private JPanel createShopCard(
            String name,
            String description,
            java.util.function.Supplier<Integer> baseCostSupplier,
            Icon icon,
            Runnable buyAction,
            java.util.function.Supplier<Boolean> soldOut,
            java.util.function.Supplier<Boolean> recommended
    ) {
        JPanel card = new JPanel(new BorderLayout(4, 4));
        card.setBorder(BorderFactory.createTitledBorder(name));
        card.setBackground(new Color(245, 247, 250));

        JLabel iconLabel = new JLabel(icon);
        iconLabel.setHorizontalAlignment(SwingConstants.CENTER);
        card.add(iconLabel, BorderLayout.WEST);

        JLabel textLabel = new JLabel();
        card.add(textLabel, BorderLayout.CENTER);

        JButton buyButton = new JButton("Buy");
        buyButton.addActionListener(e -> {
            buyAction.run();
            refreshUi();
        });

        Timer stateTimer = new Timer(250, e -> {
            int baseCost = baseCostSupplier.get();
            int cost = discountedCost(baseCost);
            boolean blocked = soldOut.get();
            buyButton.setEnabled(gold >= cost && !blocked);
            buyButton.setText(blocked ? "Maxed" : "Buy");
            String rec = recommended.get() ? "<br/><font color='#2f8f3a'><b>Recommended</b></font>" : "";
            textLabel.setText("<html>" + description + "<br/>Cost: " + cost + "g" + rec + "</html>");
        });
        stateTimer.start();
        card.addMouseListener(new java.awt.event.MouseAdapter() {
            @Override
            public void mouseEntered(java.awt.event.MouseEvent e) {
                card.setBackground(new Color(230, 241, 255));
            }

            @Override
            public void mouseExited(java.awt.event.MouseEvent e) {
                card.setBackground(new Color(245, 247, 250));
            }
        });
        card.addHierarchyListener(e -> {
            if (!card.isDisplayable()) {
                stateTimer.stop();
            }
        });

        card.add(buyButton, BorderLayout.EAST);
        return card;
    }

    private static class CatchBarPanel extends JPanel {
        private static final Color RED_ZONE = new Color(190, 30, 30);
        private static final Color YELLOW_ZONE = new Color(224, 181, 35);
        private static final Color GREEN_ZONE = new Color(40, 173, 67);

        private final double greenWidth;
        private final double yellowWidth;
        private double markerPosition = 0.0;

        CatchBarPanel(double effectiveDifficulty) {
            this.greenWidth = Math.max(0.14, 0.30 - effectiveDifficulty * 0.06);
            this.yellowWidth = Math.max(0.16, 0.36 - effectiveDifficulty * 0.05);
            setOpaque(true);
        }

        void setMarkerPosition(double markerPosition) {
            int oldX = markerToX(this.markerPosition);
            this.markerPosition = markerPosition;
            int newX = markerToX(this.markerPosition);
            int left = Math.max(0, Math.min(oldX, newX) - 8);
            int width = Math.abs(newX - oldX) + 16;
            repaint(left, 0, Math.max(18, width), getHeight());
        }

        private int markerToX(double markerPos) {
            int pad = 24;
            int w = Math.max(1, getWidth() - pad * 2);
            return pad + (int) (w * markerPos);
        }

        @Override
        protected void paintComponent(Graphics g) {
            super.paintComponent(g);
            int pad = 24;
            int w = getWidth() - pad * 2;
            int h = 44;
            int y = (getHeight() - h) / 2;
            if (w <= 4) {
                return;
            }

            int yellowPx = (int) (w * yellowWidth);
            int greenPx = (int) (w * greenWidth);
            int sideRedPx = (w - yellowPx) / 2;
            int greenStart = pad + (w - greenPx) / 2;

            g.setColor(RED_ZONE);
            g.fillRect(pad, y, sideRedPx, h);
            g.fillRect(pad + sideRedPx + yellowPx, y, sideRedPx, h);
            g.setColor(YELLOW_ZONE);
            g.fillRect(pad + sideRedPx, y, yellowPx, h);
            g.setColor(GREEN_ZONE);
            g.fillRect(greenStart, y, greenPx, h);
            g.setColor(Color.DARK_GRAY);
            g.drawRect(pad, y, w, h);

            int markerX = markerToX(markerPosition);
            g.setColor(Color.BLACK);
            g.fillRect(markerX - 2, y - 9, 4, h + 18);
        }
    }

    private static class CatchScenePanel extends JPanel {
        private final BufferedImage background;
        private final BufferedImage rod;
        private double markerPosition = 0.0;
        private int pullFrames = 0;
        private int failFrames = 0;
        private int tick = 0;

        CatchScenePanel(BufferedImage background, BufferedImage rod) {
            this.background = background;
            this.rod = rod;
            setOpaque(true);
        }

        void setMarkerPosition(double markerPosition) {
            this.markerPosition = markerPosition;
            repaint();
        }

        void playPullAnimation() {
            pullFrames = 10;
        }

        void playFailAnimation() {
            failFrames = 16;
        }

        void tick() {
            tick++;
            if (pullFrames > 0) {
                pullFrames--;
            }
            if (failFrames > 0) {
                failFrames--;
            }
        }

        @Override
        protected void paintComponent(Graphics g) {
            super.paintComponent(g);
            Graphics2D g2 = (Graphics2D) g.create();
            int w = getWidth();
            int h = getHeight();

            if (background != null) {
                g2.drawImage(background, 0, 0, w, h, null);
            } else {
                g2.setColor(new Color(92, 156, 210));
                g2.fillRect(0, 0, w, h);
            }

            int shake = failFrames > 0 ? (tick % 2 == 0 ? -4 : 4) : 0;
            int waterY = (int) (h * 0.70);
            g2.setColor(new Color(20, 95, 170, 170));
            g2.fillRect(0, waterY, w, h - waterY);

            if (rod != null) {
                int rodW = Math.max(200, w / 3);
                int rodH = Math.max(48, h / 7);
                int rodX = 35 + shake;
                int rodY = h - rodH - 34;
                double angle = -0.32 + markerPosition * 0.42;
                if (pullFrames > 0) {
                    angle -= 0.30;
                }
                AffineTransform old = g2.getTransform();
                g2.rotate(angle, rodX + 12.0, rodY + rodH - 6.0);
                g2.drawImage(rod, rodX, rodY, rodW, rodH, null);
                g2.setTransform(old);
            }

            int fishX = (int) (w * (0.2 + markerPosition * 0.6)) + shake;
            int fishY = (int) (h * 0.75 + Math.sin(tick * 0.28) * 6);
            int hookX = (int) (w * 0.32) + shake;
            int hookY = (int) (h * 0.56);
            g2.setColor(new Color(238, 238, 238, 190));
            g2.drawLine(hookX, hookY, fishX - 10, fishY);
            g2.setColor(new Color(235, 240, 245));
            g2.fillOval(fishX - 16, fishY - 8, 32, 16);
            int[] tx = {fishX - 16, fishX - 26, fishX - 26};
            int[] ty = {fishY, fishY - 8, fishY + 8};
            g2.fillPolygon(tx, ty, 3);
            g2.setColor(new Color(32, 48, 64));
            g2.fillOval(fishX + 6, fishY - 2, 3, 3);
            for (int i = 0; i < 4; i++) {
                int bx = fishX - 20 + i * 8;
                int by = fishY - 20 - ((tick + i * 4) % 12);
                g2.setColor(new Color(220, 240, 255, 120));
                g2.fillOval(bx, by, 4, 4);
            }

            if (failFrames > 0) {
                g2.setColor(new Color(170, 24, 24, 70));
                g2.fillRect(0, 0, w, h);
            }
            g2.dispose();
        }
    }

    private void consumeBuffDurations() {
        if (luckBuffCasts > 0) {
            luckBuffCasts--;
        }
        if (valueBuffCasts > 0) {
            valueBuffCasts--;
        }
    }

    private void updateWorldCycle() {
        castsSinceWeatherUpdate = 0;
        int weatherRoll = random.nextInt(100);
        if (weatherRoll < 55) {
            weather = "Clear";
        } else if (weatherRoll < 85) {
            weather = "Rain";
        } else {
            weather = "Storm";
        }

        int moonRoll = random.nextInt(100);
        if (moonRoll < 8) {
            moonPhase = "BlueMoon";
        } else if (moonRoll < 16) {
            moonPhase = "BloodMoon";
        } else {
            moonPhase = "Normal";
        }

        logAreaMaybe("- Weather changed to " + weather + ", moon is " + moonPhase + ".");
    }

    private void ensureLocationAccess() {
        if ("River".equals(currentLocation) && !isLocationUnlocked("River")) {
            currentLocation = "Pond";
        }
        if ("Ocean".equals(currentLocation) && !isLocationUnlocked("Ocean")) {
            currentLocation = "Pond";
            if (isLocationUnlocked("River")) {
                currentLocation = "River";
            }
        }
        if ("VolcanicBay".equals(currentLocation) && !isLocationUnlocked("VolcanicBay")) {
            currentLocation = "Ocean";
            if (!isLocationUnlocked("Ocean")) {
                currentLocation = "River";
            }
            if (!isLocationUnlocked("River")) {
                currentLocation = "Pond";
            }
        }
    }

    private boolean isLocationUnlocked(String location) {
        return switch (location) {
            case "Pond" -> true;
            case "River" -> fishCaught >= 12 && totalCatchValue >= 450;
            case "Ocean" -> fishCaught >= 30 && totalCatchValue >= 1500;
            case "VolcanicBay" -> fishCaught >= 60 && totalCatchValue >= 4200;
            default -> false;
        };
    }

    private void chooseLocation() {
        List<String> options = new ArrayList<>();
        options.add("Pond (Always)");
        options.add((isLocationUnlocked("River") ? "River" : "River [Locked]") + " (12 fish, 450 value)");
        options.add((isLocationUnlocked("Ocean") ? "Ocean" : "Ocean [Locked]") + " (30 fish, 1500 value)");
        options.add((isLocationUnlocked("VolcanicBay") ? "VolcanicBay" : "VolcanicBay [Locked]") + " (60 fish, 4200 value)");

        String selected = (String) JOptionPane.showInputDialog(
                null,
                "Choose a location:",
                "Locations",
                JOptionPane.PLAIN_MESSAGE,
                null,
                options.toArray(),
                options.get(0)
        );
        if (selected == null) {
            return;
        }
        if (selected.contains("[Locked]")) {
            statusLabel.setText("Location still locked. Catch more fish/value to unlock.");
            return;
        }
        if (selected.startsWith("Pond")) currentLocation = "Pond";
        if (selected.startsWith("River")) currentLocation = "River";
        if (selected.startsWith("Ocean")) currentLocation = "Ocean";
        if (selected.startsWith("VolcanicBay")) currentLocation = "VolcanicBay";
        statusLabel.setText("Travelled to " + currentLocation + ".");
        refreshUi();
    }

    private void openShop() {
        JDialog dialog = new JDialog((Frame) null, "Harbor Shop", true);
        dialog.setSize(980, 560);
        dialog.setLocationRelativeTo(null);
        dialog.setLayout(new BorderLayout(8, 8));
        TraderNpc featuredTrader = TraderNpc.fromDisplayName(getBestFriendTrader());

        JLabel header = new JLabel("Gold: " + gold + " | Discount: " + (int) Math.round((1.0 - getShopDiscountMultiplier()) * 100) + "% | Next Rod Skin: " + getNextRodUnlockText());
        header.setBorder(BorderFactory.createEmptyBorder(8, 10, 4, 10));
        dialog.add(header, BorderLayout.NORTH);

        JPanel grid = new JPanel(new GridLayout(2, 4, 8, 8));
        grid.setBorder(BorderFactory.createEmptyBorder(6, 10, 6, 10));
        grid.add(createShopCard("Rod Upgrade", "Unlocks stronger rod and next skin", () -> 200 + (rodLevel * 70),
                getScaledIcon("assets/ui/upgrade_rod.png", 42, 42), this::buyRodUpgrade, () -> rodLevel >= MAX_ROD_LEVEL, () -> rodLevel < 4));
        grid.add(createShopCard("Bait Upgrade", "Increases bait level for better catches", () -> 140 + baitLevel * 45,
                getScaledIcon("assets/ui/upgrade_bait.png", 42, 42), this::buyBaitUpgrade, () -> baitLevel >= 5, () -> baitLevel < 3));
        grid.add(createShopCard("Luck Buff", "Adds +5 luck-boosted casts", () -> 90,
                getScaledIcon("assets/ui/potion_luck.png", 42, 42), this::buyLuckBuff, () -> false, () -> luckBuffCasts < 3));
        grid.add(createShopCard("Value Buff", "Adds +5 value-boosted casts", () -> 90,
                getScaledIcon("assets/ui/potion_value.png", 42, 42), this::buyValueBuff, () -> false, () -> valueBuffCasts < 3));
        grid.add(createShopCard("Stamina Potion", "Restores 30 stamina instantly", () -> 40,
                getScaledIcon("assets/ui/potion_stamina.png", 42, 42), this::buyStaminaPotion, () -> stamina >= MAX_STAMINA, () -> stamina < 45));
        grid.add(createShopCard("Master Reel Mod", "Special item: boosts rod assist", () -> 430 + specialRodPower * 140,
                getScaledIcon("assets/ui/upgrade_rod.png", 42, 42), this::buySpecialRodMod, () -> specialRodPower >= 4 || relationTinkerKai < 14, () -> relationTinkerKai >= 14));
        JButton soundToggle = new JButton("Sound: " + (soundsEnabled ? "On" : "Off"));
        soundToggle.addActionListener(e -> {
            soundsEnabled = !soundsEnabled;
            soundToggle.setText("Sound: " + (soundsEnabled ? "On" : "Off"));
            playUiSound("ok");
        });
        JPanel soundCard = new JPanel(new BorderLayout());
        soundCard.setBorder(BorderFactory.createTitledBorder("Audio"));
        soundCard.add(new JLabel("UI and catch feedback sounds"), BorderLayout.CENTER);
        soundCard.add(soundToggle, BorderLayout.SOUTH);
        grid.add(soundCard);

        JPanel nextSkinCard = new JPanel(new BorderLayout(4, 4));
        nextSkinCard.setBorder(BorderFactory.createTitledBorder("Next Skin Preview"));
        JLabel nextSkinImage = new JLabel(getRodSkinIconForLevel(Math.min(MAX_ROD_LEVEL, rodLevel + 1)));
        nextSkinImage.setHorizontalAlignment(SwingConstants.CENTER);
        JLabel nextSkinText = new JLabel(getNextRodUnlockText(), SwingConstants.CENTER);
        nextSkinCard.add(nextSkinImage, BorderLayout.CENTER);
        nextSkinCard.add(nextSkinText, BorderLayout.SOUTH);
        grid.add(nextSkinCard);

        dialog.add(grid, BorderLayout.CENTER);
        JPanel traderPanel = new JPanel(new BorderLayout(4, 4));
        traderPanel.setBorder(BorderFactory.createTitledBorder("Featured Seller"));
        JLabel portrait = new JLabel(getScaledIcon(featuredTrader.portraitPath, 96, 96));
        portrait.setHorizontalAlignment(SwingConstants.CENTER);
        JLabel sellerName = new JLabel(featuredTrader.displayName, SwingConstants.CENTER);
        JLabel dialogueLabel = new JLabel("<html><i>" + featuredTrader.randomDialogue(random) + "</i></html>", SwingConstants.CENTER);
        traderPanel.add(portrait, BorderLayout.NORTH);
        traderPanel.add(sellerName, BorderLayout.CENTER);
        traderPanel.add(dialogueLabel, BorderLayout.SOUTH);
        dialog.add(traderPanel, BorderLayout.WEST);

        JButton closeButton = new JButton("Close Shop");
        closeButton.addActionListener(e -> dialog.dispose());
        JPanel bottom = new JPanel(new FlowLayout(FlowLayout.RIGHT));
        bottom.add(closeButton);
        dialog.add(bottom, BorderLayout.SOUTH);

        Timer refreshTimer = new Timer(250, e -> {
            header.setText("Gold: " + gold + " | Discount: " + (int) Math.round((1.0 - getShopDiscountMultiplier()) * 100) + "% | Next Rod Skin: " + getNextRodUnlockText());
            nextSkinText.setText(getNextRodUnlockText());
            nextSkinImage.setIcon(getRodSkinIconForLevel(Math.min(MAX_ROD_LEVEL, rodLevel + 1)));
        });
        refreshTimer.start();
        dialog.addWindowListener(new WindowAdapter() {
            @Override
            public void windowClosed(WindowEvent e) {
                refreshTimer.stop();
            }
        });
        dialog.setVisible(true);
        refreshTimer.stop();
        refreshUi();
    }

    private void buyRodUpgrade() {
        if (rodLevel >= MAX_ROD_LEVEL) {
            statusLabel.setText("Rod already max level.");
            playUiSound("error");
            return;
        }
        int baseCost = 200 + (rodLevel * 70);
        spendGold(baseCost, "Rod upgrade", () -> rodLevel++);
    }

    private void buyBaitUpgrade() {
        if (baitLevel >= 5) {
            statusLabel.setText("Bait already max level.");
            playUiSound("error");
            return;
        }
        spendGold(140 + baitLevel * 45, "Bait upgrade", () -> baitLevel++);
    }

    private void buyLuckBuff() {
        spendGold(90, "Luck buff", () -> luckBuffCasts += 5);
    }

    private void buyValueBuff() {
        spendGold(90, "Value buff", () -> valueBuffCasts += 5);
    }

    private void buyStaminaPotion() {
        spendGold(40, "Stamina potion", () -> stamina = Math.min(MAX_STAMINA, stamina + 30));
    }

    private void buySpecialRodMod() {
        if (specialRodPower >= 4) {
            statusLabel.setText("Special rod mod already maxed.");
            playUiSound("error");
            return;
        }
        spendGold(430 + specialRodPower * 140, "special rod mod", () -> specialRodPower++);
    }

    private void spendGold(int cost, String thing, Runnable apply) {
        int finalCost = discountedCost(cost);
        if (gold < finalCost) {
            statusLabel.setText("Not enough gold for " + thing + ".");
            playUiSound("error");
            return;
        }
        gold -= finalCost;
        apply.run();
        statusLabel.setText("Bought " + thing + " for " + finalCost + " gold.");
        logArea.append("- Shop: bought " + thing + " (" + finalCost + ")\n");
        playUiSound("ok");
    }

    private void sellInventoryInShop() {
        if (inventory.isEmpty()) {
            statusLabel.setText("Inventory is empty.");
            return;
        }
        int sum = 0;
        for (FishResult fish : inventory) {
            sum += fish.value;
        }
        inventory.clear();
        gold += sum;
        lifetimeGoldEarned += sum;
        statusLabel.setText("Sold inventory in shop for " + sum + " gold.");
        logArea.append("- Sold all fish in shop: +" + sum + " gold\n");
        refreshUi();
    }

    private void generateTradeOffer() {
        if (inventory.isEmpty()) {
            statusLabel.setText("Catch fish first before making a trade offer.");
            playUiSound("error");
            return;
        }
        String[] traders = {TraderNpc.MARINA.displayName, TraderNpc.BROKER_FINN.displayName, TraderNpc.TINKER_KAI.displayName};
        String trader = (String) JOptionPane.showInputDialog(
                null,
                "Choose trader:",
                "Trade Contacts",
                JOptionPane.PLAIN_MESSAGE,
                null,
                traders,
                traders[0]
        );
        if (trader == null) {
            return;
        }
        TraderNpc npc = TraderNpc.fromDisplayName(trader);
        showTraderDialogue(npc, "Trade Contact");
        String[] minRarityOptions = {"Common", "Uncommon", "Rare"};
        String minRarity = minRarityOptions[random.nextInt(minRarityOptions.length)];
        double relationBonus = Math.min(0.30, getRelationship(trader) * 0.01);
        double multiplier = 1.35 + random.nextDouble() * 0.40 + relationBonus;
        if ("Broker Finn".equals(trader)) {
            multiplier += 0.08;
        }
        if ("Marina".equals(trader) && "Rare".equals(minRarity)) {
            minRarity = random.nextBoolean() ? "Uncommon" : "Common";
        }
        long expiresAtMs = System.currentTimeMillis() + 45000L;
        tradeOffer = new TradeOffer(npc.displayName, minRarity, multiplier, expiresAtMs);
        statusLabel.setText("Trade offer created.");
        logArea.append("- New trade offer by " + trader + ": " + minRarity + "+ fish at x"
                + String.format(Locale.US, "%.2f", multiplier) + " for 45s\n");
        playUiSound("ok");
        refreshUi();
    }

    private void tryTradeOffer() {
        if (tradeOffer == null || !tradeOffer.isActive()) {
            statusLabel.setText("No active trade offer.");
            tradeOffer = null;
            playUiSound("error");
            refreshUi();
            return;
        }
        int sold = 0;
        int rawValue = 0;
        for (int i = inventory.size() - 1; i >= 0; i--) {
            FishResult fish = inventory.get(i);
            if (rarityRank(fish.rarity) >= rarityRank(tradeOffer.minRarity)) {
                sold++;
                rawValue += fish.value;
                inventory.remove(i);
            }
        }
        if (sold == 0) {
            statusLabel.setText("No fish matched trade rarity " + tradeOffer.minRarity + "+.");
            playUiSound("error");
            return;
        }
        int payout = (int) Math.round(rawValue * tradeOffer.multiplier);
        gold += payout;
        lifetimeGoldEarned += payout;
        addRelationship(tradeOffer.traderName, 1 + sold / 3);
        logArea.append("- Trade sold " + sold + " fish at x"
                + String.format(Locale.US, "%.2f", tradeOffer.multiplier)
                + " for +" + payout + " gold (Rel " + tradeOffer.traderName + ": "
                + getRelationship(tradeOffer.traderName) + ")\n");
        statusLabel.setText("Trade successful: +" + payout + " gold.");
        playUiSound("ok");
        showTraderDialogue(TraderNpc.fromDisplayName(tradeOffer.traderName), "Trade Completed");
        tradeOffer = null;
        refreshUi();
    }

    private int rarityRank(String rarity) {
        return switch (rarity) {
            case "Common" -> 1;
            case "Uncommon" -> 2;
            case "Rare" -> 3;
            case "Epic" -> 4;
            case "Legendary" -> 5;
            case "Mythic" -> 6;
            default -> 1;
        };
    }

    private void rest() {
        if (!restManager.canRestNow()) {
            statusLabel.setText("Rest is on cooldown for " + restManager.secondsRemaining() + "s.");
            playUiSound("error");
            refreshUi();
            return;
        }
        stamina = Math.min(MAX_STAMINA, stamina + 25);
        long cooldownMs = Math.round(38000 * PlayerArchetype.fromName(playerCharacter).castSpeedMultiplier);
        restManager.triggerCooldownMs(cooldownMs);
        statusLabel.setText("Rested. Stamina restored. Cooldown started.");
        refreshUi();
    }

    private FishResult runFishEngine() {
        try {
            Path root = Path.of("").toAbsolutePath();
            Path enginePath = root.resolve("bin").resolve("fish_engine");
            if (!Files.exists(enginePath)) {
                return FishResult.error("C++ engine binary not found at " + enginePath + ". Run ./build.sh first.");
            }
            Path savePath = root.resolve(SAVE_PATH);

            ProcessBuilder processBuilder = new ProcessBuilder(
                    enginePath.toString(),
                    savePath.toString(),
                    currentLocation,
                    String.valueOf(Math.max(1, rodLevel)),
                    String.valueOf(baitLevel),
                    weather,
                    moonPhase,
                    String.valueOf(luckBuffCasts),
                    String.valueOf(valueBuffCasts)
            );
            processBuilder.redirectErrorStream(true);
            Process process = processBuilder.start();

            try (BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()))) {
                String line = reader.readLine();
                int exitCode = process.waitFor();

                if (exitCode != 0) {
                    return FishResult.error("Engine exited with code " + exitCode);
                }
                if (line == null || line.isBlank()) {
                    return FishResult.error("Engine produced no output.");
                }

                return FishResult.parse(line);
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return FishResult.error(e.getMessage());
        } catch (IOException e) {
            return FishResult.error(e.getMessage());
        }
    }

    private void handleExit(JFrame frame) {
        int choice = JOptionPane.showOptionDialog(
                frame,
                "Save progress before exit?",
                "Exit Fishing Game",
                JOptionPane.YES_NO_CANCEL_OPTION,
                JOptionPane.QUESTION_MESSAGE,
                null,
                new String[]{"Save and Exit", "Exit without Saving", "Cancel"},
                "Save and Exit"
        );

        if (choice == 0) {
            if (saveProgress()) {
                frame.dispose();
            } else {
                JOptionPane.showMessageDialog(frame, "Failed to save progress.", "Save Error", JOptionPane.ERROR_MESSAGE);
            }
        } else if (choice == 1) {
            frame.dispose();
        }
    }

    private void saveProgressAndReport() {
        if (saveProgress()) {
            statusLabel.setText("Progress saved.");
            logArea.append("- Progress saved to " + SAVE_PATH + "\n");
        } else {
            statusLabel.setText("Save failed.");
            logArea.append("- Failed to save progress.\n");
        }
    }

    private void maybeShowNewPlayerGuide(Component parent) {
        int choice = JOptionPane.showConfirmDialog(
                parent,
                "New save detected. Open quick guide?",
                "Fishing Game Guide",
                JOptionPane.YES_NO_OPTION
        );
        if (choice != JOptionPane.YES_OPTION) {
            return;
        }
        String guide = """
                How to play:
                1) Cast line -> pull 3 times to identify fish, then keep reeling in green zone.
                2) Catch fish -> fish goes into inventory.
                3) Sell inventory in shop for gold.
                4) Upgrade rod and bait; rods improve catch assist and unlock skins.
                5) Unlock locations by catching/value milestones in Locations screen.
                6) Generate trade offers and pick traders to build relationship.
                   Higher relationship gives better trade multipliers and lower shop prices.
                7) Special items become available with strong relationships.
                8) Rest restores stamina, but it has a cooldown timer.
                """;
        JOptionPane.showMessageDialog(parent, guide, "Quick Guide", JOptionPane.INFORMATION_MESSAGE);
    }

    private void chooseStartingCharacter(Component parent) {
        PlayerArchetype[] options = PlayerArchetype.values();
        String[] labels = new String[options.length];
        for (int i = 0; i < options.length; i++) {
            PlayerArchetype a = options[i];
            labels[i] = a.displayName
                    + " | Gold " + a.startingGold
                    + " | Rod L" + a.startingRodLevel + " (worn starter)"
                    + " | Bait L" + a.startingBaitLevel
                    + " | Cast " + (int) Math.round((1.0 - a.castSpeedMultiplier) * 100) + "%";
        }
        String selected = (String) JOptionPane.showInputDialog(
                parent,
                "Choose your character:",
                "New Save - Character Select",
                JOptionPane.PLAIN_MESSAGE,
                null,
                labels,
                labels[0]
        );
        if (selected == null) {
            selected = labels[0];
        }
        PlayerArchetype chosen = options[0];
        for (PlayerArchetype a : options) {
            if (selected.startsWith(a.displayName)) {
                chosen = a;
                break;
            }
        }
        applyStartingArchetype(chosen);
    }

    private void applyStartingArchetype(PlayerArchetype archetype) {
        playerCharacter = archetype.displayName;
        gold = archetype.startingGold;
        rodLevel = archetype.startingRodLevel;
        baitLevel = archetype.startingBaitLevel;
        stamina = Math.min(MAX_STAMINA, archetype.startingStamina);
        statusLabel.setText("Character chosen: " + playerCharacter + ".");
        refreshUi();
    }

    private boolean saveProgress() {
        try {
            if (SAVE_PATH.getParent() != null) {
                Files.createDirectories(SAVE_PATH.getParent());
            }
            String json = buildProgressJson();
            Files.writeString(SAVE_PATH, json, StandardCharsets.UTF_8);
            return true;
        } catch (IOException e) {
            return false;
        }
    }

    private String buildProgressJson() {
        StringBuilder sb = new StringBuilder();
        sb.append("{\n");
        sb.append("  \"fishCaught\": ").append(fishCaught).append(",\n");
        sb.append("  \"totalCatchValue\": ").append(totalCatchValue).append(",\n");
        sb.append("  \"gold\": ").append(gold).append(",\n");
        sb.append("  \"lifetimeGoldEarned\": ").append(lifetimeGoldEarned).append(",\n");
        sb.append("  \"stamina\": ").append(stamina).append(",\n");
        sb.append("  \"rodLevel\": ").append(rodLevel).append(",\n");
        sb.append("  \"baitLevel\": ").append(baitLevel).append(",\n");
        sb.append("  \"playerCharacter\": \"").append(escapeJson(playerCharacter)).append("\",\n");
        sb.append("  \"currentLocation\": \"").append(escapeJson(currentLocation)).append("\",\n");
        sb.append("  \"weather\": \"").append(escapeJson(weather)).append("\",\n");
        sb.append("  \"moonPhase\": \"").append(escapeJson(moonPhase)).append("\",\n");
        sb.append("  \"luckBuffCasts\": ").append(luckBuffCasts).append(",\n");
        sb.append("  \"valueBuffCasts\": ").append(valueBuffCasts).append(",\n");
        sb.append("  \"specialRodPower\": ").append(specialRodPower).append(",\n");
        sb.append("  \"soundsEnabled\": ").append(soundsEnabled ? 1 : 0).append(",\n");
        sb.append("  \"relationMarina\": ").append(relationMarina).append(",\n");
        sb.append("  \"relationBrokerFinn\": ").append(relationBrokerFinn).append(",\n");
        sb.append("  \"relationTinkerKai\": ").append(relationTinkerKai).append(",\n");
        sb.append("  \"nextRestAtMs\": ").append(restManager.getNextRestAtMs()).append(",\n");
        sb.append("  \"lastTenValues\": [");
        int i = 0;
        for (int value : lastTenValues) {
            if (i++ > 0) {
                sb.append(", ");
            }
            sb.append(value);
        }
        sb.append("],\n");
        sb.append("  \"inventory\": [");
        for (int idx = 0; idx < inventory.size(); idx++) {
            if (idx > 0) {
                sb.append(", ");
            }
            sb.append(fishToJson(inventory.get(idx)));
        }
        sb.append("],\n");
        sb.append("  \"leastValueFish\": ").append(fishToJson(leastValueFish)).append(",\n");
        sb.append("  \"mostValueFish\": ").append(fishToJson(mostValueFish)).append("\n");
        sb.append("}\n");
        return sb.toString();
    }

    private String fishToJson(FishResult fish) {
        if (fish == null) {
            return "null";
        }
        return "{"
                + "\"species\":\"" + escapeJson(fish.species) + "\","
                + "\"rarity\":\"" + escapeJson(fish.rarity) + "\","
                + "\"weightKg\":" + String.format(Locale.US, "%.2f", fish.weightKg) + ","
                + "\"value\":" + fish.value
                + "}";
    }

    private String escapeJson(String value) {
        return value == null ? "" : value.replace("\\", "\\\\").replace("\"", "\\\"");
    }

    private boolean loadProgress() {
        if (!Files.exists(SAVE_PATH)) {
            return false;
        }

        try {
            String json = Files.readString(SAVE_PATH, StandardCharsets.UTF_8);
            fishCaught = extractInt(json, "fishCaught", 0);
            totalCatchValue = extractInt(json, "totalCatchValue", 0);
            if (totalCatchValue == 0) {
                totalCatchValue = extractInt(json, "totalGold", 0);
            }
            gold = extractInt(json, "gold", 0);
            lifetimeGoldEarned = extractInt(json, "lifetimeGoldEarned", 0);
            stamina = extractInt(json, "stamina", MAX_STAMINA);
            rodLevel = extractInt(json, "rodLevel", 1);
            baitLevel = extractInt(json, "baitLevel", 1);
            playerCharacter = extractString(json, "playerCharacter", PlayerArchetype.ALEX.displayName);
            currentLocation = extractString(json, "currentLocation", "Pond");
            weather = extractString(json, "weather", "Clear");
            moonPhase = extractString(json, "moonPhase", "Normal");
            luckBuffCasts = extractInt(json, "luckBuffCasts", 0);
            valueBuffCasts = extractInt(json, "valueBuffCasts", 0);
            lastTenValues.clear();
            for (int value : extractIntArray(json, "lastTenValues")) {
                lastTenValues.addLast(value);
            }
            inventory.clear();
            inventory.addAll(extractFishArray(json, "inventory"));
            leastValueFish = extractFishObject(json, "leastValueFish");
            mostValueFish = extractFishObject(json, "mostValueFish");
            specialRodPower = extractInt(json, "specialRodPower", 0);
            soundsEnabled = extractInt(json, "soundsEnabled", 1) == 1;
            relationMarina = extractInt(json, "relationMarina", 0);
            relationBrokerFinn = extractInt(json, "relationBrokerFinn", 0);
            relationTinkerKai = extractInt(json, "relationTinkerKai", 0);
            restManager.setNextRestAtMs((long) extractDouble(json, "nextRestAtMs", 0.0));
            return true;
        } catch (IOException ignored) {
            // Keep defaults if load fails.
            return false;
        }
    }

    private int extractInt(String json, String field, int fallback) {
        Pattern pattern = Pattern.compile("\"" + Pattern.quote(field) + "\"\\s*:\\s*(-?\\d+)");
        Matcher matcher = pattern.matcher(json);
        if (matcher.find()) {
            return Integer.parseInt(matcher.group(1));
        }
        return fallback;
    }

    private int[] extractIntArray(String json, String field) {
        Pattern pattern = Pattern.compile("\"" + Pattern.quote(field) + "\"\\s*:\\s*\\[(.*?)\\]", Pattern.DOTALL);
        Matcher matcher = pattern.matcher(json);
        if (!matcher.find()) {
            return new int[0];
        }

        String raw = matcher.group(1).trim();
        if (raw.isEmpty()) {
            return new int[0];
        }

        String[] parts = raw.split(",");
        int[] values = new int[parts.length];
        for (int i = 0; i < parts.length; i++) {
            values[i] = Integer.parseInt(parts[i].trim());
        }
        return values;
    }

    private FishResult extractFishObject(String json, String field) {
        Pattern objectPattern = Pattern.compile("\"" + Pattern.quote(field) + "\"\\s*:\\s*(null|\\{.*?\\})", Pattern.DOTALL);
        Matcher objectMatcher = objectPattern.matcher(json);
        if (!objectMatcher.find()) {
            return null;
        }
        String objectValue = objectMatcher.group(1);
        if ("null".equals(objectValue)) {
            return null;
        }

        FishResult fish = new FishResult();
        fish.species = extractString(objectValue, "species", "Unknown");
        fish.rarity = extractString(objectValue, "rarity", "Common");
        fish.weightKg = extractDouble(objectValue, "weightKg", 0.0);
        fish.value = extractInt(objectValue, "value", 0);
        return fish;
    }

    private List<FishResult> extractFishArray(String json, String field) {
        List<FishResult> results = new ArrayList<>();
        Pattern arrayPattern = Pattern.compile("\"" + Pattern.quote(field) + "\"\\s*:\\s*\\[(.*?)]", Pattern.DOTALL);
        Matcher arrayMatcher = arrayPattern.matcher(json);
        if (!arrayMatcher.find()) {
            return results;
        }

        String block = arrayMatcher.group(1);
        Pattern objectPattern = Pattern.compile("\\{.*?}", Pattern.DOTALL);
        Matcher matcher = objectPattern.matcher(block);
        while (matcher.find()) {
            String objectText = matcher.group();
            FishResult fish = new FishResult();
            fish.species = extractString(objectText, "species", "Unknown");
            fish.rarity = extractString(objectText, "rarity", "Common");
            fish.weightKg = extractDouble(objectText, "weightKg", 0.0);
            fish.value = extractInt(objectText, "value", 0);
            results.add(fish);
        }
        return results;
    }

    private String extractString(String json, String field, String fallback) {
        Pattern pattern = Pattern.compile("\"" + Pattern.quote(field) + "\"\\s*:\\s*\"([^\"]*)\"");
        Matcher matcher = pattern.matcher(json);
        if (matcher.find()) {
            return matcher.group(1);
        }
        return fallback;
    }

    private double extractDouble(String json, String field, double fallback) {
        Pattern pattern = Pattern.compile("\"" + Pattern.quote(field) + "\"\\s*:\\s*(-?\\d+(?:\\.\\d+)?)");
        Matcher matcher = pattern.matcher(json);
        if (matcher.find()) {
            return Double.parseDouble(matcher.group(1));
        }
        return fallback;
    }

    private static class FishResult {
        String species;
        String rarity;
        double weightKg;
        int value;
        String error;

        static FishResult error(String message) {
            FishResult result = new FishResult();
            result.error = message;
            return result;
        }

        static FishResult parse(String data) {
            String[] parts = data.split("\\|");
            if (parts.length != 4) {
                return error("Unexpected output: " + data);
            }

            try {
                FishResult result = new FishResult();
                result.species = parts[0];
                result.rarity = parts[1];
                result.weightKg = Double.parseDouble(parts[2]);
                result.value = Integer.parseInt(parts[3]);
                return result;
            } catch (NumberFormatException e) {
                return error("Invalid numeric value in engine output: " + data);
            }
        }
    }

    private void logAreaMaybe(String text) {
        if (logArea != null) {
            logArea.append("- " + text + "\n");
        }
    }

    private static class TradeOffer {
        String traderName;
        String minRarity;
        double multiplier;
        long expiresAtMs;

        TradeOffer(String traderName, String minRarity, double multiplier, long expiresAtMs) {
            this.traderName = traderName;
            this.minRarity = minRarity;
            this.multiplier = multiplier;
            this.expiresAtMs = expiresAtMs;
        }

        boolean isActive() {
            return System.currentTimeMillis() < expiresAtMs;
        }

        long secondsRemaining() {
            return Math.max(0L, (expiresAtMs - System.currentTimeMillis()) / 1000L);
        }
    }
}
