package br.com.gabryel.y2022

import java.util.Scanner

private val scanner = Scanner(System.`in`)

fun getLines() = getLines { it }

fun <T> getLines(transform: (line: String) -> T) =
    generateSequence { if (scanner.hasNextLine()) scanner.nextLine().ifEmpty { null } else null }
        .mapNotNull(transform)