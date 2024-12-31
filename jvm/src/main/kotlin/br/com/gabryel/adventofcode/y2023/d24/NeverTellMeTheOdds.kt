package br.com.gabryel.adventofcode.y2023.d24

import br.com.gabryel.adventofcode.util.Direction.*
import br.com.gabryel.adventofcode.util.DoubleCoordinate
import br.com.gabryel.adventofcode.util.plusa
import br.com.gabryel.adventofcode.util.x
import br.com.gabryel.adventofcode.util.y
import br.com.gabryel.adventofcode.y2023.d24.Vector3D.Dimension
import br.com.gabryel.adventofcode.y2023.d24.Vector3D.Dimension.*
import java.math.RoundingMode.HALF_EVEN
import java.text.DecimalFormat
import java.util.*
import kotlin.math.*

fun countHailstoneFutureIntersections(lines: List<String>, min: Long, max: Long): Int {
    val hailstones = lines.map { it.toHailStone() }

    return hailstones.countFutureIntersections(min.toDouble().rangeTo(max.toDouble()))
}

/**
 * This one is a doozy, I wasn't able to solve it in the original year and was only able to finish it on 2024
 * with some support. And in contrast with day 21, which was super hard for me, this one needed a lot more
 * mathematical basis than I had to solve.
 *
 * For reference:
 * - https://www.youtube.com/watch?v=nP2ahZs40U8
 * - https://github.com/werner77/AdventOfCode/blob/master/src/main/kotlin/com/behindmedia/adventofcode/year2023/day24/Day24.kt
 *
 * Although I took some shortcuts and liberties over his code for performance, I was only able to solve
 * this problem with this video's help
 *
 * Also, there is this more efficient mathematical solution, but I am against using magic which I don't understand
 * and can't explain when I am trying to learn:
 * - https://github.com/zebalu/advent-of-code-2023/blob/4f68f96a841d85c7c797ebbc003f9edc5f65672c/aoc2023/src/main/java/io/github/zebalu/aoc2023/days/Day24.java
 * - https://github.com/DeadlyRedCube/AdventOfCode/blob/1f9d0a3e3b7e7821592244ee51bce5c18cf899ff/2023/AOC2023/D24.h#L66-L294
 *
 * Maybe if I go into linear algebra someday?
 */
fun getSumOfRockInitialPositions(lines: List<String>): Long {
    val hailstones = lines.map { it.toHailStone() }

    return hailstones.findMatching(X, Y).mapNotNull { (position, delta) ->
        println("$position - $delta")
        val hailstone0 = Hailstone(
            Vector3D(position.x().toDouble(), position.y().toDouble(), 0.0),
            Vector3D(delta.x().toDouble(), delta.y().toDouble(), 0.0),
        )

        println("$hailstone0\n")
        val intersectionTimes = hailstones.map { hailstone ->
            val t = -(hailstone.position[X] - hailstone0.position[X]) / (hailstone.velocity[X] - hailstone0.velocity[X])
            t to hailstone.position[Z] + (t * hailstone.velocity[Z])
        }.asSequence()

        if (intersectionTimes.any { it.first < 0 }) {
            return@mapNotNull null
        }

        val positionZ = intersectionTimes.windowed(2, 2) { (f, s) ->
            val timeDiff = f.first - s.first
            val diff = f.second - s.second
            val dz = (diff / timeDiff)

            (dz to f.second - (f.first * dz))
        }.firstOrNull()?.second ?: return@mapNotNull null

        println(positionZ.roundToInt())

        position.x().roundToLong() + position.y().roundToLong() + positionZ.roundToLong()
    }.first()
}

private fun List<Hailstone>.countFutureIntersections(
    range: ClosedRange<Double>,
    d1: Dimension = X,
    d2: Dimension = Y
) = withIndex().sumOf { (index, hailstone1) ->
    (index + 1..lastIndex).count { index2 ->
        hailstone1.intersect(this[index2], d1, d2, range)
    }
}

private fun Hailstone.intersect(
    hailstone2: Hailstone,
    d1: Dimension,
    d2: Dimension,
    range: ClosedRange<Double>
) = intersect(hailstone2, d1, d2)
    ?.let { it[d1] in range && it[d2] in range }
    ?: false

private fun Hailstone.intersect(hailstone2: Hailstone, d1: Dimension, d2: Dimension) =
    moveToIntersectionPoint(hailstone2, d1, d2)?.second
        ?.takeIf { isInFuture(it) && hailstone2.isInFuture(it) }

private fun Hailstone.isInFuture(new: Vector3D) =
    (new.x - position.x).sign == velocity.x.sign

private fun String.toHailStone(): Hailstone {
    val (position, velocity) = split("@")
        .map { it.replace(" ", "") }
        .map { it.toVector3D() }

    return Hailstone(position, velocity)
}

private fun String.toVector3D(): Vector3D {
    val (x, y, z) = split(",").map { it.toLong().toDouble() }
    return Vector3D(x, y, z)
}

private data class Hailstone(val position: Vector3D, val velocity: Vector3D) {
    val normalized = EnumMap<Dimension, Vector3D>(Dimension::class.java)

    fun removeVelocity(dimension: Dimension, value: Double) = copy(velocity = velocity.remove(dimension, value))

    fun moveToIntersectionPoint(other: Hailstone, d1: Dimension, d2: Dimension): Pair<Double, Vector3D>? {
        val intersept1 = findInterceptFor(d2, d1) ?: return null
        val intersept2 = other.findInterceptFor(d2, d1) ?: return null

        val slope1 = findSlope(d2, d1) ?: return null
        val slope2 = other.findSlope(d2, d1) ?: return null

        if ((slope1 - slope2) == 0.0) return null

        val d1AtIntersection = (intersept2 - intersept1) / (slope1 - slope2)
        val moved = moveTo(d1AtIntersection, d1) ?: return null
        return d1AtIntersection to moved
    }

    fun moveNanoseconds(nano: Double) = position + (velocity * nano)

    private fun findInterceptFor(intercept: Dimension, zeroAt: Dimension): Double? {
        return moveTo(0.0, zeroAt)?.let { it[intercept] }
    }

    private fun findSlope(slope: Dimension, normal: Dimension) = normalized.getOrPut(normal) {
        velocity.normalizedTo(normal)
    }?.get(slope)

    private fun moveTo(value: Double, dimension: Dimension): Vector3D? {
        if (value == 0.0 && velocity[dimension] == 0.0)
            return position

        val movement = getNanoSecondsToMove(value, dimension) ?: return null
        return moveNanoseconds(movement)
    }

    private fun getNanoSecondsToMove(movement: Double, dimension: Dimension): Double? {
        if (velocity[dimension] == 0.0) return null
        return (movement - position[dimension]) / velocity[dimension]
    }
}

private data class Vector3D(val x: Double, val y: Double, val z: Double) {
    enum class Dimension { X, Y, Z }

    operator fun get(dimension: Dimension) = when (dimension) {
        X -> x
        Y -> y
        Z -> z
    }

    fun normalizedTo(dimension: Dimension) = if (this[dimension] != 0.0) {
        copy(x = x / this[dimension], y = y / this[dimension], z = z / this[dimension])
    } else null

    fun remove(dimension: Dimension, amount: Double) = when (dimension) {
        X -> copy(x = x - amount)
        Y -> copy(y = y - amount)
        Z -> copy(z = z - amount)
    }

    operator fun plus(other: Vector3D) = Vector3D(x + other.x, y + other.y, z + other.z)

    operator fun times(quantity: Double) = Vector3D(x * quantity, y * quantity, z * quantity)

    override fun toString() = "[${x.stringified()}; ${y.stringified()}; ${z.stringified()}]"
}

private fun Double.stringified(): String {
    val decimalPlaces = "".padStart(5, '#')
    val df = DecimalFormat("#.$decimalPlaces")
    df.roundingMode = HALF_EVEN

    return df.format(this)
}

private fun List<Hailstone>.findMatching(d1: Dimension, d2: Dimension): Sequence<Pair<DoubleCoordinate, DoubleCoordinate>> {
    val intersections = 8
    val neededToPass = min(size -2, 6)

    return generateSpiralSequence().mapNotNull { (v1, v2) ->
        val base = this[0]

        (1 until minOf(intersections, size)).map { index2 ->
            val hailstone1 = base.removeVelocity(d1, v1).removeVelocity(d2, v2)
            val hailstone2 = this[index2].removeVelocity(d1, v1).removeVelocity(d2, v2)
            hailstone1.moveToIntersectionPoint(hailstone2, d1, d2)
        }.let { items ->
            val nonNull = items.filterNotNull()

            if (nonNull.size < neededToPass)
                return@mapNotNull null

            val first = nonNull.first()
            if (nonNull.any { first.areTooFar(it, d1, d2, 0.3) })
                return@mapNotNull null

            val (_, intersection) = first
            (intersection[d1] to intersection[d2]) to (v1 to v2)
        }
    }
}

private fun Pair<Double, Vector3D>.areTooFar(right: Pair<Double, Vector3D>, d1: Dimension, d2: Dimension, tolerance: Double) =
    (second[d1] - right.second[d1]).absoluteValue > tolerance || (second[d2] - right.second[d2]).absoluteValue > tolerance

private fun generateSpiralSequence() = generateSequence(1) { it + 1 }.flatMap {
    sequenceOf(1 to UP, ((it * 2) - 1) to RIGHT, it * 2 to DOWN, it * 2 to LEFT, it * 2 to UP)
        .flatMap { (amount, dir) -> (1..amount).asSequence().map { dir } }
}.scan(0.0 to 0.0) { acc, dir -> acc.plusa(dir) }
