package br.com.gabryel.adventofcode.util

import kotlin.math.absoluteValue

typealias TCoordinate<T> = Pair<T, T>

typealias Coordinate = Pair<Int, Int>

typealias DoubleCoordinate = Pair<Double, Double>

data class IntCoordinate(val x: Int, val y: Int)

fun <T> TCoordinate<T>.x() = first

fun <T> TCoordinate<T>.y() = second

infix fun Coordinate.getManhattanDistance(other: Coordinate) = (x() - other.x()).absoluteValue + (y() - other.y()).absoluteValue

infix fun Coordinate.distanceTo(other: Coordinate) = (x() - other.x()) to (y() - other.y())

operator fun IntCoordinate.plus(other: Coordinate) = IntCoordinate(x + other.x(), y + other.y())

operator fun IntCoordinate.plus(other: Direction) = IntCoordinate(x + other.vector.x(), y + other.vector.y())

operator fun Coordinate.plus(other: Coordinate) = (x() + other.x()) to (y() + other.y())

operator fun Coordinate.plus(other: Direction) = this + other.vector

fun DoubleCoordinate.plusDouble(other: Direction) = (x() + other.vector.x()) to (y() + other.vector.y())

operator fun <T: Number> TCoordinate<T>.times(other: Int) = (x().toInt() * other) to (y().toInt() * other)

operator fun <T: Number> TCoordinate<T>.times(other: Long) = (x().toLong() * other) to (y().toLong() * other)

operator fun <T: Number> TCoordinate<T>.div(other: Long) = (x().toLong() * other).toInt() to (y().toLong() * other).toInt()

operator fun <T: Number> TCoordinate<T>.div(other: Int): Coordinate = (x().toInt() / other) to (y().toInt() / other)

operator fun Coordinate.rem(other: Int) = (x() % other) to (y() % other)

operator fun Coordinate.rem(other: Long) = (x() % other).toInt() to (y() % other).toInt()

fun Coordinate.getAdjacent() = Direction.entries.map { this + it }

infix fun Coordinate.bindTo(dimensions: Coordinate) = (x().mod(dimensions.x())) to (y().mod(dimensions.y()))

operator fun Coordinate.contains(other: Coordinate) = other.x() in (0 until x()) && other.y() in (0 until y())

val ZERO = 0 to 0
