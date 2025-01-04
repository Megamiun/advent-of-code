package br.com.gabryel.adventofcode.util

fun List<Int>.leastCommonMultiplier() = reduce { acc, curr -> leastCommonMultiplier(acc, curr) }

fun List<Long>.leastCommonMultiplier() = reduce { acc, curr -> leastCommonMultiplier(acc, curr) }

fun leastCommonMultiplier(a: Int, b: Int) = (a * b) / greatestCommonDivisor(a, b)

fun leastCommonMultiplier(a: Long, b: Long) = (a * b) / greatestCommonDivisor(a, b)

fun greatestCommonDivisor(a: Int, b: Int): Int {
    if (b == 0) return a
    return greatestCommonDivisor(b, a % b)
}

fun greatestCommonDivisor(a: Long, b: Long): Long {
    if (b == 0L) return a
    return greatestCommonDivisor(b, a % b)
}