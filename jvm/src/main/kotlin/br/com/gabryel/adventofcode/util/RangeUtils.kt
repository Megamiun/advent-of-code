package br.com.gabryel.adventofcode.util

import kotlin.math.max

fun IntRange.size() = max((last - first) + 1, 0)

fun LongRange.size() = max((last - first) + 1, 0)

fun IntRange.intersects(other: IntRange) =
    first in other || last in other || other.first in this || other.last in this

fun IntRange.displace(num: Int) = first + num..last + num
