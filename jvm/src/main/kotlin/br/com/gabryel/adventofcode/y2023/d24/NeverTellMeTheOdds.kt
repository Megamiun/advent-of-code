package br.com.gabryel.adventofcode.y2023.d24

import br.com.gabryel.adventofcode.y2023.d24.Vector3D.Dimension
import br.com.gabryel.adventofcode.y2023.d24.Vector3D.Dimension.*
import br.com.gabryel.adventofcode.y2023.readLines
import java.math.BigDecimal
import java.math.BigInteger
import java.math.MathContext
import java.math.RoundingMode.HALF_EVEN
import java.text.DecimalFormat
import java.util.PriorityQueue

fun main() {
    listOf("sample"/*, "input"*/).forEach { file ->
        val hailstones = readLines(24, file).map { it.toHailStone() }

        listOf(
            7.0..27.0,
            200000000000000.0..400000000000000.0
        ).forEach { range ->
//            println("[Future XY Intersections][$range][$file] ${hailstones.getFutureIntersections(range)}")
        }

        val nonSkews = hailstones.filterNSkews(5)

        val initialRock = Hailstone(Vector3D(
            0.bigDecimal(),
            0.bigDecimal(),
            0.bigDecimal()),
            Vector3D(
                10.bigDecimal(),
                10.bigDecimal(),
                10.bigDecimal()))

        val visited = PriorityQueue<Pair<Hailstone, BigDecimal>>(compareBy { it.second })
        val toVisitSet = mutableSetOf<Hailstone>()
        val toVisit = PriorityQueue<Pair<Hailstone, BigDecimal>>(compareBy { it.second }).apply {
            add(initialRock to nonSkews.map { initialRock.findDiffAtClosestPointPerAxis(it) }.sumOf {
                it.values.sumOf { it.first.pow(2) + it.second.pow(2) }
            })
        }

        var iteration = 0
        while(toVisit.isNotEmpty()) {
            val (rock, value) = toVisit.remove()

            toVisitSet.add(rock)
            visited.add(rock to value)

            println()
            println()
            println("----------")
            println("${++iteration} - $rock")
            println()

            nonSkews.map {
                println()
                println("*****")
                println("$it")
                rock.findDiffAtClosestPointPerAxis(it)
                    .onEach { (k, v) -> println("$k - ${v.first.stringified()}; ${v.second.stringified()}") }
            }

            val position = rock.position
            val velocity = rock.velocity

            val one = BigDecimal.ONE
            val oneN = BigDecimal.ONE.negate()
            val zero = BigDecimal.ZERO

            val directions = listOf(
                Vector3D(zero, zero, one),
                Vector3D(zero, zero, oneN),
                Vector3D(zero, one, one),
                Vector3D(zero, one, oneN),
                Vector3D(zero, oneN, one),
                Vector3D(zero, oneN, oneN),
                Vector3D(one, zero, zero),
                Vector3D(one, zero, one),
                Vector3D(one, zero, oneN),
                Vector3D(one, one, one),
                Vector3D(one, one, oneN),
                Vector3D(one, oneN, one),
                Vector3D(one, oneN, oneN),
                Vector3D(oneN, zero, zero),
                Vector3D(oneN, zero, one),
                Vector3D(oneN, zero, oneN),
                Vector3D(oneN, one, one),
                Vector3D(oneN, one, oneN),
                Vector3D(oneN, oneN, one),
                Vector3D(oneN, oneN, oneN),
            ).flatMap { listOf(it, it * 2.bigDecimal(), it * 3.bigDecimal(), it * 11.bigDecimal()) }

            toVisit.addAll(
                directions.flatMap {
                    listOf(rock.copy(position = position + it), rock.copy(velocity = velocity + it), rock.copy(position = position + it, velocity = velocity + it))
                }.filter { r -> Dimension.entries.all { r.velocity[it].abs() > BigDecimal("0.001") } && r !in toVisitSet }
                .map { newRock ->
                    val sum = nonSkews.flatMap { newRock.findDiffAtClosestPointPerAxis(it).values }
                        .sumOf { (f, s) -> f.pow(2) + s.pow(2) }
                    newRock to sum
            })
        }
    }
}

private fun List<Hailstone>.filterNSkews(desired: Int): List<Hailstone> {
    val nonSkews = mutableListOf<Hailstone>()

    for (hailstone in this) {
        if (nonSkews.none { hailstone.normalized == it.normalized }) {
            nonSkews += hailstone

            if (nonSkews.size == desired) break
        }
    }
    return nonSkews
}

private fun List<Hailstone>.getFutureIntersections(
    range: ClosedFloatingPointRange<BigDecimal>,
    d1: Dimension = X,
    d2: Dimension = Y
) = withIndex().sumOf { (index, hailstone1) ->
    (index + 1..lastIndex).count { index2 ->
        val hailstone2 = this[index2]

        val intersectionPoint = hailstone1.moveToIntersectionPoint(hailstone2, d1, d2)

                intersectionPoint[d1] in range &&
                intersectionPoint[d2] in range &&
                hailstone1.isInFuture(intersectionPoint) &&
                hailstone2.isInFuture(intersectionPoint)
    }
}

private fun Hailstone.isInFuture(new: Vector3D) =
    (new.x - position.x).signum() == velocity.x.signum()

private fun String.toHailStone(): Hailstone {
    val (position, velocity) = split("@")
        .map { it.replace(" ", "") }
        .map { it.toVector3D() }

    return Hailstone(position, velocity)
}

private fun String.toVector3D(): Vector3D {
    val (x, y, z) = split(",").map { it.toLong().bigDecimal() }
    return Vector3D(x, y, z)
}

private data class Hailstone(val position: Vector3D, val velocity: Vector3D) {

    val normalized by lazy {
        mapOf(
            X to velocity.normalizedTo(X),
            Y to velocity.normalizedTo(Y),
            Z to velocity.normalizedTo(Z),
        )
    }

    fun findDiffAtClosestPointPerAxis(other: Hailstone) = mapOf(X to (Y to Z), Y to (X to Z), Z to (X to Y))
        .mapValues { (_, values) -> moveToIntersectionPoint(other, values.first, values.second) }
        .mapValues { (dimension, intersection) ->
            val basisDimension = when(dimension) {
                X -> Y
                Y -> Z
                Z -> X
            }
            val basisValue = intersection[basisDimension]

            val originalT = getNanoSecondsToMove(basisValue, basisDimension)
            val otherT = other.getNanoSecondsToMove(basisValue, basisDimension)

            val originalMoved = moveNanoseconds(originalT)
            val otherMoved = other.moveNanoseconds(otherT)

            val forDimension = originalMoved[dimension] - otherMoved[dimension]
            val time = (originalT - otherT)

            val pastCorrection = listOf(otherT, originalT).filter { it < BigDecimal.ZERO }.sumOf { it }

            forDimension to (time + pastCorrection)
        }

    fun moveToIntersectionPoint(other: Hailstone, d1: Dimension, d2: Dimension): Vector3D {
        val intersept1 = findInterceptFor(d2, d1)
        val intersept2 = other.findInterceptFor(d2, d1)

        val slope1 = findSlope(d2, d1)
        val slope2 = other.findSlope(d2, d1)

        if ((slope1 - slope2).unscaledValue() == BigInteger.ZERO) return this.position

        val d1AtIntersection = (intersept2 - intersept1) / (slope1 - slope2)
        return moveTo(d1AtIntersection, d1)
    }

    fun moveNanoseconds(nano: BigDecimal) = position + (velocity * nano)

    fun getNanoSecondsToMove(movement: BigDecimal, dimension: Dimension) =
        (movement - position[dimension]) / velocity[dimension]

    fun findInterceptFor(intercept: Dimension, zeroAt: Dimension) =
        moveTo(BigDecimal.ZERO, zeroAt)[intercept]

    private fun moveTo(value: BigDecimal, dimension: Dimension) =
        moveNanoseconds(getNanoSecondsToMove(value, dimension))

    fun findSlope(slope: Dimension, normal: Dimension) = normalized[normal]!![slope]
}

private data class Vector3D(val x: BigDecimal, val y: BigDecimal, val z: BigDecimal) {
    enum class Dimension { X, Y, Z }

    operator fun get(dimension: Dimension) = when (dimension) {
        X -> x
        Y -> y
        Z -> z
    }

    fun normalizedTo(dimension: Dimension) =
        copy(x = x / this[dimension], y = y / this[dimension], z = z / this[dimension])

    fun calculateDistanceTo(other: Vector3D, vararg dimensions: Dimension = arrayOf(X, Y, Z)) =
        calculateSquareDistanceTo(other, *dimensions).sqrt(MathContext.DECIMAL128)

    fun calculateSquareDistanceTo(other: Vector3D, vararg dimensions: Dimension = arrayOf(X, Y, Z)) =
        dimensions.sumOf { squareDimensionDiff(other, it) }

    private fun squareDimensionDiff(other: Vector3D, dimension: Dimension) =
        (this[dimension] - other[dimension]).pow(2)

    operator fun plus(other: Vector3D) = Vector3D(x + other.x, y + other.y, z + other.z)

    operator fun times(quantity: BigDecimal) = Vector3D(x * quantity, y * quantity, z * quantity)

    override fun toString() = "[${x.stringified()}; ${y.stringified()}; ${z.stringified()}]"
}

private fun Long.bigDecimal() = toBigDecimal().setScale(32)

private fun Int.bigDecimal() = toBigDecimal().setScale(32)

private fun BigDecimal.stringified(): String {
    val decimalPlaces = "".padStart(5, '#')
    val df = DecimalFormat("#.$decimalPlaces")
    df.roundingMode = HALF_EVEN

    return df.format(this)
}
