package me.hydos.quiltlangrust.entrypoint;

import net.fabricmc.api.DedicatedServerModInitializer;

public class RustDedicatedServerModInitializer extends RustNativeInitializer implements DedicatedServerModInitializer{

    public RustDedicatedServerModInitializer(String libName, String modid) {
        super(libName, modid);
    }

    @Override
    public void onInitializeServer() {
        runNativeDedicatedServerInitializer(libName, modid);
    }

    public native void runNativeDedicatedServerInitializer(String libPath, String modid);
}
