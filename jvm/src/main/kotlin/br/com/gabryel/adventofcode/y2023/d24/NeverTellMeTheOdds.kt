package br.com.gabryel.adventofcode.y2023.d24

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.y2023.d24.Vector3D.Dimension
import br.com.gabryel.adventofcode.y2023.d24.Vector3D.Dimension.*
import java.math.BigDecimal
import java.math.BigInteger
import java.math.MathContext
import java.math.RoundingMode.HALF_EVEN
import java.text.DecimalFormat

fun main() {
    listOf(
        "sample" to BigDecimal(7).rangeTo(BigDecimal(27)),
        "input" to BigDecimal(200000000000000).rangeTo(BigDecimal(400000000000000))
    ).forEach { (file, range) ->
        val hailstones = readLines(2023, 24, file).map { it.toHailStone() }

        println("[Never Tell me The Odds - Future Intersections][$file] ${hailstones.getFutureIntersections(range)}")
//        println("[Never Tell me The Odds - Single Intersection][$file] ${hailstones.getFutureIntersections(range)}")
    }
}

private fun List<Hailstone>.getFutureIntersections(
    range: ClosedRange<BigDecimal>,
    d1: Dimension = X,
    d2: Dimension = Y
) = withIndex().sumOf { (index, hailstone1) ->
    (index + 1..lastIndex).count { index2 ->
        val hailstone2 = this[index2]

        val intersectionPoint = hailstone1.moveToIntersectionPoint(hailstone2, d1, d2)

        intersectionPoint[d1] in range && intersectionPoint[d2] in range &&
                hailstone1.isInFuture(intersectionPoint) && hailstone2.isInFuture(intersectionPoint)
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
