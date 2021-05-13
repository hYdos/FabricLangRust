package me.hydos.quiltlangrust.entrypoint;

public abstract class RustNativeInitializer {

    public final String modid;
    public final String libName;

    public RustNativeInitializer(String libName, String modid) {
        this.libName = libName;
        this.modid = modid;
    }
}
