package br.com.gabryel.adventofcode.y2022

import java.util.Scanner

private val scanner = Scanner(System.`in`)

fun getGroupsOfLines() = generateSequence { getLines().toList().ifEmpty { null } }

fun getLines() = getLines { it }

fun <T> getLines(transform: (line: String) -> T) =
    generateSequence { if (scanner.hasNextLine()) scanner.nextLine().ifEmpty { null } else null }
        .takeWhile(String::isNotBlank)
        .mapNotNull(transform)
