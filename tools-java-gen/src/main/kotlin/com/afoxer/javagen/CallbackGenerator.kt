package com.afoxer.javagen

import com.squareup.javapoet.*
import java.io.Serializable
import javax.lang.model.element.Modifier


class CallbackGenerator(val desc: TraitDesc, val pkg: String) {
    fun generate(): JavaFile.Builder {
        val builder = TypeSpec.interfaceBuilder(this.desc.name)
                .addSuperinterface(Serializable::class.java)
                .addModifiers(Modifier.PUBLIC)

        val methods = desc.methods;
        for (method in methods) {
            val methodSpec = MethodSpec.methodBuilder(method.name)
                    .addModifiers(Modifier.PUBLIC)
                    .addModifiers(Modifier.ABSTRACT)
                    .returns(mapType(method.return_type, method.return_sub_type))

            val args = method.args
            for (arg in args) {
                when (arg.ty) {
                    AstType.STRUCT -> {
                        methodSpec.addParameter(ClassName.get(pkg, arg.origin_ty), arg.name)
                    }
                    AstType.VEC -> {
                        when (arg.sub_ty) {
                            AstType.STRUCT -> {
                                val subType = ClassName.get(pkg, arg.origin_ty.replace("Vec<", "").replace(">", ""))
                                val param = ParameterSpec.builder(ArrayTypeName.of(subType), arg.name)
                                methodSpec.addParameter(param.build())
                            }
                            else -> {
                                val param = ParameterSpec.builder(mapType(arg.ty, arg.sub_ty), arg.name)
                                methodSpec.addParameter(param.build())
                            }
                        }
                    }
                    else -> {
                        val param = ParameterSpec.builder(mapType(arg.ty, AstType.VOID), arg.name)
                        methodSpec.addParameter(param.build())
                    }
                }
            }

            builder.addMethod(methodSpec.build())
        }

        return JavaFile.builder(this.pkg, builder.build())
    }
}