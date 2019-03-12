package com.afoxer.javagen

import java.io.File

fun main(args: Array<String>) {
    if (args.size < 5) {
        println("Usage: javabind package_name so_name ext_libs ast_path out_path")
        return
    }

    val pkg = args[0]
    val soName = args[1]
    val extLibs = args[2]
    val astPath = args[3]
    val outPath = args[4]

    val parser = AstParser(astPath)
    val astResult = parser.parseAst()
    val traits = astResult.traits
    val structs = astResult.structs;

    val callbacks = traits.filter { it.is_callback }
    for (desc in traits) {
        if (desc.is_callback) {
            val generator = CallbackGenerator(desc, pkg)
            val javaFile = generator.generate()
            javaFile.build().writeTo(File(outPath))
        } else {
            val generator = TraitGenerator(desc, pkg, soName, extLibs, callbacks)
            val javaFile = generator.generate()
            javaFile.build().writeTo(File(outPath))
        }
    }

    for (struct in structs) {
        val generator = StructGenerator(struct, pkg)
        val javaFile = generator.generate()
        javaFile.build().writeTo(File(outPath))
    }
}