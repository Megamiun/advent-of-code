package br.com.gabryel.adventofcode.y2023

object ResourceReader

fun readLines(day: Int, file: String, keepBlanks: Boolean = false) =
    readFile(day, file)
        .lines()
        .filter { keepBlanks || it.isNotBlank() }

fun readFile(day: Int, file: String) =
    ResourceReader.javaClass.getResourceAsStream("d${day.toString().padStart(2, '0')}/$file")
        ?.bufferedReader()?.readText()
        ?: throw IllegalStateException("""¨d$day/$file¨ not found""")
