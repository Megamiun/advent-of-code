package br.com.gabryel.y2022

fun main() {
    val pairs = getAllElfPairs()

    println("Containing Pairs: ${pairs.count { (first, second) -> first in second || second in first }}")
    println("Intersecting Pairs: ${pairs.count { (first, second) -> first.intersect(second).isNotEmpty() }}")
}

private infix operator fun IntRange.contains(other: IntRange) =
    first in other && last in other

private fun getAllElfPairs() = getLines { line ->
    val (first, second) = line.split(",").map { pair ->
        val (start, ending) = pair.split("-")
        start.toInt()..ending.toInt()
    }

    first to second
}.toList()

