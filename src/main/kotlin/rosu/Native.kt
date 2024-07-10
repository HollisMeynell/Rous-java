package rosu

import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardCopyOption

class Native private constructor() {
    companion object {
        @JvmStatic
        val instance by lazy {
            val native = Native()
            native.loadLib
            native
        }
    }

    val loadLib by lazy {
        val os: String = System.getProperty("os.name")
        val type = when {
            os.contains("windows", ignoreCase = true) -> "dll"
            os.contains("mac", ignoreCase = true) -> "dylib"
            os.contains("linux", ignoreCase = true) -> "so"
            else -> throw Error("Unsupported OS")
        }
        val name = "rosu_pp_java.${type}"
        val lib = Native::class.java.getResourceAsStream("/lib/${name}")
        lib?.use {
            val tmpDirPath = Path.of(
                System.getenv("ROSU_LIB_PATH")
                    ?: (System.getProperty("java.io.tmpdir") + "/jlib")
            )
            if (Files.isDirectory(tmpDirPath).not()) {
                Files.createDirectory(tmpDirPath)
            }
            val f = tmpDirPath.resolve(name)
            Files.copy(lib, f, StandardCopyOption.REPLACE_EXISTING)
            Runtime.getRuntime().addShutdownHook(Thread {
                try {
                    Files.delete(f)
                } catch (ignore: Exception) {

                }
            })
            System.load(f.toString())
        }
    }

    @JvmName("calculate")
    external fun calculate(localMap: ByteArray, source: ByteArray): ByteArray

    @JvmName("getCalculateIterator")
    external fun getCalculateIterator(localMap: ByteArray, mapAttr: ByteArray): ByteArray

    @JvmName("calculateIterator")
    external fun calculateIterator(ptr: Long, score: ByteArray): ByteArray

    @JvmName("collectionCalculate")
    external fun collectionCalculate(ptr: Long): Boolean

    @JvmName("createCollection")
    external fun createCollection(name: String): ByteArray

    @JvmName("setCollectionMap")
    external fun setCollectionMap(ptr: Long, map: String): Boolean

    @JvmName("newCollectionList")
    external fun newCollectionList(ptr: Long): ByteArray

    @JvmName("appendCollectionList")
    external fun appendCollectionList(collectionList: ByteArray, ptr: Long): ByteArray
}