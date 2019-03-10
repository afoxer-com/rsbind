package com.bytedance.ee.bear.javagen

import com.squareup.javapoet.*
import com.squareup.javapoet.ArrayTypeName
import java.io.Serializable
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicLong
import javax.lang.model.element.Modifier


class TraitGenerator(val desc: TraitDesc, val pkg: String, val soName: String, val extLibs: String, val callbacks: List<TraitDesc>) {
    fun generate(): JavaFile.Builder {
        // loadLibrary
        val staticBlock = CodeBlock.builder().addStatement("System.loadLibrary(\"${soName}\")")
        val extLibArray = extLibs.split(",")
        for (extLib in extLibArray) {
            if (extLib.isBlank()) {
                continue
            }
            staticBlock.addStatement("System.loadLibrary(\"${extLib}\")")
        }

        val builder = TypeSpec.classBuilder(this.desc.name)
                .addModifiers(Modifier.PUBLIC)
                .addSuperinterface(ClassName.get(Serializable::class.java))
                .addStaticBlock(staticBlock.build())

        // global properties.
        buildGlobal(builder)

        var selectedCallbacks = mutableListOf<TraitDesc>()

        // normal functions
        val methods = desc.methods;
        for (method in methods) {
            val methodSpec = MethodSpec.methodBuilder(method.name)
                    .addModifiers(Modifier.PUBLIC)
                    .addModifiers(Modifier.STATIC)
            if (method.return_type == AstType.VOID) {
                //skip
            } else if (method.return_type == AstType.STRUCT) {
                methodSpec.returns(ClassName.get(pkg, method.origin_return_ty))
            } else if (method.return_type == AstType.VEC) {
                var subType = mapType(method.return_sub_type, method.return_sub_type).box()
                if (method.return_sub_type == AstType.STRUCT) {
                    val clsName = method.origin_return_ty.replace("Vec<", "").replace(">", "")
                    subType = ClassName.get(pkg, clsName)
                }
                methodSpec.returns(ArrayTypeName.of(subType))
            } else {
                methodSpec.returns(mapType(method.return_type, method.return_sub_type))
            }

            val args = method.args
            for (arg in args) {
                if (arg.ty == AstType.VOID) {
                    // skip
                } else if (arg.ty == AstType.CALLBACK) {
                    val param = ParameterSpec.builder(ClassName.get(pkg, arg.origin_ty), arg.name)
                    methodSpec.addParameter(param.build())
                    val callback = callbacks.filter { it.name == arg.origin_ty }
                    if (!selectedCallbacks.contains(callback[0])) {
                        selectedCallbacks.add(callback[0])
                    }
                } else if (arg.ty == AstType.VEC) {
                    val param = ParameterSpec.builder(ArrayTypeName.of(mapType(arg.sub_ty, AstType.VOID)), arg.name)
                    methodSpec.addParameter(param.build())
                } else {
                    val param = ParameterSpec.builder(mapType(arg.ty, AstType.VOID), arg.name)
                    methodSpec.addParameter(param.build())
                }
            }

            // arguments convert
            for (arg in args) {
                if (arg.ty == AstType.VOID) {
                    //skip
                } else if (arg.ty == AstType.CALLBACK) {
                    methodSpec.addStatement("long ${arg.name}_callback_index = globalIndex.incrementAndGet()")
                            .addStatement("globalCallbacks.put(${arg.name}_callback_index, ${arg.name})")
                            .addStatement("long r_${arg.name} = ${arg.name}_callback_index")
                } else if (arg.ty == AstType.BOOLEAN) {
                    methodSpec.addStatement("int r_${arg.name} = ${arg.name} ? 1 : 0")
                } else if (arg.ty == AstType.VEC) {
                    methodSpec.addStatement("String r_${arg.name} = \$T.toJSONString(${arg.name})", ClassName.get("com.alibaba.fastjson", "JSON"))
                } else {
                    methodSpec.addStatement("\$T r_${arg.name} = ${arg.name}", mapType(arg.ty, AstType.VOID))
                }
            }

            // call native method
            val subType = mapType(method.return_sub_type, method.return_sub_type)
            val returnType = if (method.return_type == AstType.VEC) {
                mapType(AstType.STRING, method.return_sub_type, true)
            } else if (method.return_type == AstType.STRUCT) {
                ClassName.get(String::class.java)
            } else {
                mapType(method.return_type, method.return_sub_type, true)
            }

            var callMethodStatement = if (method.return_type == AstType.VOID) {
                "native_\$L("
            } else {
                "\$T ret = native_\$L("
            }

            for ((index, arg) in args.withIndex()) {
                if (index == args.size - 1) {
                    callMethodStatement = "${callMethodStatement}r_${arg.name}"
                } else {
                    callMethodStatement = "${callMethodStatement}r_${arg.name}, "
                }
            }
            callMethodStatement = "$callMethodStatement)"

            if (method.return_type == AstType.VOID) {
                methodSpec.addStatement(callMethodStatement, method.name)
            } else {
                methodSpec.addStatement(callMethodStatement, returnType, method.name)
            }

            // return type convert
            if (method.return_type == AstType.VOID) {
                // skip
            } else if (method.return_type == AstType.VEC) {
                var subType = mapType(method.return_sub_type, method.return_sub_type)
                if (method.return_sub_type == AstType.STRUCT) {
                    val clsName = method.origin_return_ty.replace("Vec<", "").replace(">", "")
                    subType = ClassName.get(pkg, clsName)
                }
                methodSpec.addStatement("\$T<\$T> list = \$T.parseArray(ret, \$T.class)",
                        ClassName.get("java.util", "List"), subType.box(),
                        ClassName.get("com.alibaba.fastjson", "JSON"), subType.box())
                        .addStatement("\$T[] array = new \$T[list.size()]", subType.box(), subType.box())
                        .addStatement("return list.toArray(array)")
            } else if (method.return_type == AstType.BOOLEAN) {
                methodSpec.addStatement("return ret > 0 ? true : false")
            } else if (method.return_type == AstType.STRUCT) {
                methodSpec.addStatement("return \$T.parseObject(ret, \$L.class)",
                        ClassName.get("com.alibaba.fastjson", "JSON"),
                        method.origin_return_ty);
            } else {
                methodSpec.addStatement("return ret");
            }

            builder.addMethod(methodSpec.build())
        }

        // callbacks invoke functions
        for (callback in selectedCallbacks) {
            for (method in callback.methods) {
                val methodBuilder = MethodSpec.methodBuilder("invoke_${callback.name}_${method.name}")
                        .addModifiers(Modifier.PUBLIC)
                        .addModifiers(Modifier.STATIC)
                if (method.return_type != AstType.VOID) {
                    methodBuilder.returns(mapType(method.return_type, AstType.VOID, true))
                }

                var argCalls = ""
                methodBuilder.addParameter(TypeName.LONG, "index")
                for ((index, arg) in method.args.withIndex()) {
                    when (arg.ty) {
                        AstType.VEC, AstType.STRUCT -> {
                            methodBuilder.addParameter(ClassName.get("java.lang", "String"), arg.name)
                        }
                        else -> {
                            methodBuilder.addParameter(mapType(arg.ty, AstType.VOID, true), arg.name)
                        }
                    }

                    if (index == method.args.size - 1) {
                        argCalls = "${argCalls}j_${arg.name}"
                    } else {
                        argCalls = "${argCalls}j_${arg.name},"
                    }
                }

                // argument convert
                for (arg in method.args) {
                    when (arg.ty) {
                        AstType.BOOLEAN -> {
                            methodBuilder.addStatement("boolean j_${arg.name} = ${arg.name} > 0 ? true : false")
                        }
                        AstType.STRUCT -> {
                            methodBuilder.addStatement("${arg.origin_ty} j_${arg.name} = \$T.parseObject(${arg.name}, ${arg.origin_ty}.class)", ClassName.get("com.alibaba.fastjson", "JSON"))
                        }
                        AstType.VEC -> {
                            var subType = mapType(arg.sub_ty, arg.sub_ty)
                            if (arg.sub_ty == AstType.STRUCT) {
                                val clsName = arg.origin_ty.replace("Vec<", "").replace(">", "")
                                subType = ClassName.get(pkg, clsName)
                            }
                            methodBuilder.addStatement("\$T<\$T> ${arg.name}_list = \$T.parseArray(${arg.name}, \$T.class)",
                                    ClassName.get("java.util", "List"), subType.box(),
                                    ClassName.get("com.alibaba.fastjson", "JSON"), subType.box())
                                    .addStatement("\$T[] ${arg.name}_array = new \$T[${arg.name}_list.size()]", subType.box(), subType.box())
                                    .addStatement("\$T[] j_${arg.name} = ${arg.name}_list.toArray(${arg.name}_array)", subType.box())
                        }
                        else -> {
                            methodBuilder.addStatement("\$T j_${arg.name} = ${arg.name}", mapType(arg.ty, arg.sub_ty))
                        }
                    }
                }

                methodBuilder
                        .addCode(CodeBlock.builder().addStatement("${callback.name} callback = (${callback.name}) globalCallbacks.get(index)").build())
                if (method.return_type == AstType.VOID) {
                    methodBuilder.addCode(CodeBlock.builder().addStatement("callback.${method.name}($argCalls)").build())
                } else {
                    methodBuilder.addCode(CodeBlock.builder().addStatement("\$T result = callback.${method.name}($argCalls)", mapType(method.return_type, method.return_sub_type)).build())
                }

                if (method.return_type == AstType.BOOLEAN) {
                    methodBuilder.addStatement("return result ? 1 : 0")
                } else if (method.return_type == AstType.VOID) {
                    // skip
                } else {
                    methodBuilder.addStatement("return result")
                }

                builder.addMethod(methodBuilder.build())
            }
        }

        // native functions
        buildNativeMethods(methods, builder)

        return JavaFile.builder(this.pkg, builder.build())
    }

    /**
     *   build global properties and methods.
     */
     fun buildGlobal(builder: TypeSpec.Builder) {
        builder.addField(FieldSpec.builder(ClassName.get(AtomicLong::class.java), "globalIndex", Modifier.PRIVATE, Modifier.STATIC)
                .initializer("new AtomicLong(0)")
                .build())

        builder.addField(FieldSpec.builder(ParameterizedTypeName.get(ClassName.get(ConcurrentHashMap::class.java), TypeName.LONG.box(), ClassName.get(Object::class.java)),
                "globalCallbacks", Modifier.PRIVATE, Modifier.STATIC)
                .initializer("new \$T<>()", ClassName.get(ConcurrentHashMap::class.java))
                .build())

        builder.addMethod(MethodSpec.methodBuilder("free_callback")
                .addModifiers(Modifier.PUBLIC)
                .addModifiers(Modifier.STATIC)
                .addParameter(TypeName.LONG, "index")
                .addCode(CodeBlock.builder().addStatement("globalCallbacks.remove(index)").build())
                .build())
     }

    /**
    * build native methods for accessing .so
    */
    fun buildNativeMethods(methods: Array<MethodDesc>, builder: TypeSpec.Builder) {
        for (method in methods) {
            val methodSpec = MethodSpec.methodBuilder("native_" + method.name)
                    .addModifiers(Modifier.PRIVATE)
                    .addModifiers(Modifier.STATIC)
                    .addModifiers(Modifier.NATIVE);
            if(method.return_type == AstType.VOID) {
                // skip
            } else if (method.return_type == AstType.VEC) {
                methodSpec.returns(mapType(AstType.STRING, AstType.VOID, true))
            } else if (method.return_type == AstType.STRUCT) {
                methodSpec.returns(ClassName.get(String::class.java))
            } else {
                methodSpec.returns(mapType(method.return_type, AstType.VOID, true))
            }

            val args = method.args;
            for (arg in args) {
                if (arg.ty == AstType.VOID) {
                    // skip
                } else if (arg.ty == AstType.CALLBACK) {
                    val param = ParameterSpec.builder(mapType(AstType.LONG, AstType.VOID, true), arg.name)
                    methodSpec.addParameter(param.build())
                } else if (arg.ty == AstType.VEC) {
                    val param = ParameterSpec.builder(mapType(AstType.STRING, AstType.VOID, true), arg.name)
                    methodSpec.addParameter(param.build())
                } else {
                    val param = ParameterSpec.builder(mapType(arg.ty, AstType.VOID, true), arg.name)
                    methodSpec.addParameter(param.build())
                }
            }

            builder.addMethod(methodSpec.build())
        }
    }
}