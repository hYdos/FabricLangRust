package me.hydos.quiltlangrust;

import net.fabricmc.loader.api.FabricLoader;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

public class QuiltLangRust {

    public static final String MODID = "quilt_lang_rust";
    public static final File NATIVES_DIR = new File(FabricLoader.getInstance().getGameDir().toAbsolutePath() + "/natives");
    public static boolean isLoaded;

    /**
     * Load's the rust side of the language adapter.
     */
    public static void tryLoadRust() {
        if (!isLoaded) {
            Path extractedNativeLoc = new File(NATIVES_DIR, "libQuiltLangRust.so").toPath();
            Path nativeInJar = FabricLoader.getInstance().getModContainer(MODID).get().getPath("linux-x86-64/libQuiltLangRust.so");
            try {
                Files.deleteIfExists(extractedNativeLoc);
                Files.createDirectories(Paths.get(String.valueOf(NATIVES_DIR)));
                Files.copy(nativeInJar, extractedNativeLoc);
                System.load(extractedNativeLoc.toAbsolutePath().toString());
            } catch (UnsatisfiedLinkError | IOException e) {
                System.err.println("Native code library failed to load.");
                e.printStackTrace();
                System.exit(-1);
            }
            isLoaded = true;
        }
    }
}
