package com.fishinggame;

public class RestManager {
    private long nextRestAtMs = 0L;

    public boolean canRestNow() {
        return System.currentTimeMillis() >= nextRestAtMs;
    }

    public void triggerCooldownMs(long cooldownMs) {
        nextRestAtMs = System.currentTimeMillis() + Math.max(0L, cooldownMs);
    }

    public long secondsRemaining() {
        return Math.max(0L, (nextRestAtMs - System.currentTimeMillis()) / 1000L);
    }

    public void setNextRestAtMs(long value) {
        nextRestAtMs = value;
    }

    public long getNextRestAtMs() {
        return nextRestAtMs;
    }
}
