package me.hydos.quiltlangrust;

import me.hydos.quiltlangrust.entrypoint.RustClientModInitializer;
import me.hydos.quiltlangrust.entrypoint.RustDedicatedServerModInitializer;
import me.hydos.quiltlangrust.entrypoint.RustModInitializer;
import net.fabricmc.loader.api.LanguageAdapter;
import net.fabricmc.loader.api.LanguageAdapterException;
import net.fabricmc.loader.api.ModContainer;

public class RustLangAdapter implements LanguageAdapter {

    public static final String FILE_SUFFIX = System.getProperty("os.name").contains("Win") ? ".dll" : ".so";

    @Override
    public <T> T create(ModContainer mod, String entrypointName, Class<T> type) throws LanguageAdapterException {
        String libName = entrypointName + FILE_SUFFIX;
        String modid = mod.getMetadata().getId();

        QuiltLangRust.tryLoadRust();
        switch (type.getSimpleName()) {
            case "ModInitializer" -> {
                return type.cast(new RustModInitializer(libName, modid));
            }
            case "ClientModInitializer" -> {
                return type.cast(new RustClientModInitializer(libName, modid));
            }
            case "DedicatedServerModInitializer" -> {
                return type.cast(new RustDedicatedServerModInitializer(libName, modid));
            }
            default -> throw new LanguageAdapterException("Can't handle initializer of type: " + type.getSimpleName());
        }
    }
}
