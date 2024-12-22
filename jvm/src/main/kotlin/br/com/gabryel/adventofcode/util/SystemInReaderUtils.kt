package br.com.gabryel.adventofcode.util

import java.util.Scanner

private val scanner = Scanner(System.`in`)

fun getGroupsFromSystemIn() = generateSequence { getLinesFromSystemIn().toList().ifEmpty { null } }

fun getLinesFromSystemIn() = getLinesFromSystemIn { it }

fun <T> getLinesFromSystemIn(transform: (line: String) -> T) =
    generateSequence { if (scanner.hasNextLine()) scanner.nextLine().ifEmpty { null } else null }
        .takeWhile(String::isNotBlank)
        .mapNotNull(transform)
