package com.fishinggame;

public enum PlayerArchetype {
    ALEX("Alex", 90, 0, 1, 100, 1.00, 1.00, 1.00),
    TOMMY("Tommy", 70, 0, 1, 100, 0.96, 1.12, 0.98),
    RYAN("Ryan", 120, 0, 1, 95, 1.05, 1.00, 0.92);

    public final String displayName;
    public final int startingGold;
    public final int startingRodLevel;
    public final int startingBaitLevel;
    public final int startingStamina;
    public final double castSpeedMultiplier;
    public final double relationGainMultiplier;
    public final double shopCostMultiplier;

    PlayerArchetype(String displayName, int startingGold, int startingRodLevel, int startingBaitLevel, int startingStamina,
                    double castSpeedMultiplier, double relationGainMultiplier, double shopCostMultiplier) {
        this.displayName = displayName;
        this.startingGold = startingGold;
        this.startingRodLevel = startingRodLevel;
        this.startingBaitLevel = startingBaitLevel;
        this.startingStamina = startingStamina;
        this.castSpeedMultiplier = castSpeedMultiplier;
        this.relationGainMultiplier = relationGainMultiplier;
        this.shopCostMultiplier = shopCostMultiplier;
    }

    public static PlayerArchetype fromName(String name) {
        for (PlayerArchetype a : values()) {
            if (a.displayName.equalsIgnoreCase(name)) return a;
        }
        return ALEX;
    }
}
