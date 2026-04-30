package com.fishinggame;

import java.util.Random;

public enum TraderNpc {
    MARINA("Marina", "assets/characters/marina.png", new String[]{
            "Fresh fish, fair tides. Bring me quality and I pay back.",
            "Storm is coming. Trade now while prices are hot.",
            "You reel clean. I like reliable anglers."
    }),
    BROKER_FINN("Broker Finn", "assets/characters/finn.png", new String[]{
            "Margins matter. Bring me rarity and we both profit.",
            "Quick hands, quick deal. Show me your best lot.",
            "You keep the supply flowing, I keep the rates flowing."
    }),
    TINKER_KAI("Tinker Kai", "assets/characters/kai.png", new String[]{
            "I tune rods that sing through the line.",
            "Bring materials and trust; I can upgrade miracles.",
            "You fish hard. I can make your rod hit harder."
    });

    public final String displayName;
    public final String portraitPath;
    private final String[] dialoguePool;

    TraderNpc(String displayName, String portraitPath, String[] dialoguePool) {
        this.displayName = displayName;
        this.portraitPath = portraitPath;
        this.dialoguePool = dialoguePool;
    }

    public String randomDialogue(Random random) {
        return dialoguePool[random.nextInt(dialoguePool.length)];
    }

    public static TraderNpc fromDisplayName(String name) {
        for (TraderNpc npc : values()) {
            if (npc.displayName.equalsIgnoreCase(name)) return npc;
        }
        return MARINA;
    }
}
