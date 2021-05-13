package me.hydos.quiltlangrust.entrypoint;

import net.fabricmc.api.ModInitializer;

public class RustModInitializer extends RustNativeInitializer implements ModInitializer {

    public RustModInitializer(String libName, String modid) {
        super(libName, modid);
    }

    @Override
    public void onInitialize() {
        runNativeInitializer(libName, modid);
    }

    public native void runNativeInitializer(String libPath, String modid);
}
