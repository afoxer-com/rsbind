package com.afoxer.javagen

import com.squareup.javapoet.FieldSpec
import com.squareup.javapoet.JavaFile
import com.squareup.javapoet.TypeSpec
import java.io.Serializable
import javax.lang.model.element.Modifier

class StructGenerator(val desc: StructDesc, val pkg: String) {
    fun generate(): JavaFile.Builder {
        val builder = TypeSpec.classBuilder(this.desc.name)
                .addSuperinterface(Serializable::class.java)
                .addModifiers(Modifier.PUBLIC)

        val fields = desc.fields
        for (field in fields) {
            val fieldBuilder = FieldSpec.builder(mapType(field.ty, field.sub_ty, false), field.name, Modifier.PUBLIC)
            builder.addField(fieldBuilder.build())
        }
        return JavaFile.builder(this.pkg, builder.build())
    }
}