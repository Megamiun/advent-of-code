package br.com.gabryel.adventofcode.util

import kotlin.math.max
import kotlin.math.min

fun IntRange.size() = max((last - first) + 1, 0)

fun LongRange.size() = max((last - first) + 1, 0)

fun IntRange.removeInclusive(other: IntRange): List<IntRange> =
    listOf(this.first..min(other.first, this.last), max(this.first, other.last)..this.last)
        .filterNot(IntRange::isEmpty)

fun IntRange.merge(other: IntRange) =
    min(other.first, first).. max(other.last, last)

infix fun IntRange.intersects(other: IntRange) =
    first in other || last in other || other.first in this || other.last in this

infix fun IntRange.isInside(other: IntRange) =
    first in other && last in other

fun IntRange.displace(num: Int) = first + num..last + num
