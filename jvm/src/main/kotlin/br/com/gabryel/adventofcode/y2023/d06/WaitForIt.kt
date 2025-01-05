package br.com.gabryel.adventofcode.y2023.d06

import kotlin.math.log10
import kotlin.math.pow

fun getProductOfPossibilities(lines: List<String>) =  lines.parse()
    .map { (time, record) -> calculateWinPossibilities(time, record) }
    .reduce(Long::times)

fun getKerningMistakePossibilities(lines: List<String>) = lines.parse()
    .reduce { (timeAcc, recordAcc), (timeSuffix, recordSuffix) ->
        timeAcc.concat(timeSuffix) to recordAcc.concat(recordSuffix)
    }.let { (time, record) -> calculateWinPossibilities(time, record) }

private fun Long.concat(suffix: Long): Long {
    val suffixLength = log10(suffix.toDouble()).toInt() + 1
    val multiplier = 10.0.pow(suffixLength).toInt()
    return (this * multiplier) + suffix
}

private fun calculateWinPossibilities(time: Long, record: Long): Long {
    val timeFrame = (1 until time)

    val firstWin = timeFrame
        .first { accelerationTime -> beatsRecord(time, accelerationTime, record) }

    val lastWin = timeFrame
        .last { accelerationTime -> beatsRecord(time, accelerationTime, record) }

    return (lastWin - firstWin) + 1
}

private fun beatsRecord(time: Long, accelerationTime: Long, record: Long) =
    (time - accelerationTime) * accelerationTime > record

private fun List<String>.parse(): List<Pair<Long, Long>> {
    val (time, record) = map { it.split("\\s+".toRegex()).drop(1).map(String::toLong) }

    return time.zip(record)
}
