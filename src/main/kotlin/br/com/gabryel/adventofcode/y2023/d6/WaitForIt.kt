package br.com.gabryel.adventofcode.y2023.d6

import br.com.gabryel.adventofcode.y2023.readLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val (time, record) = readLines(6, file)
            .map { it.split("\\s+".toRegex()).drop(1) }

        val timeWithRecords = time.zip(record)

        println("[Possibilites Product][$file] ${timeWithRecords.getProductOfPossibilities()}")
        println("[Kerning Possibilites][$file] ${timeWithRecords.getKerningMistakePossibilities()}")
    }
}

private fun List<Pair<String, String>>.getKerningMistakePossibilities() = reversed()
    .reduce { (time, record), (timePrefix, recordPrefix) -> (timePrefix + time) to (recordPrefix + record) }
    .let { (time, record) -> calculateWinPossibilities(time.toLong(), record.toLong()) }


private fun List<Pair<String, String>>.getProductOfPossibilities() = map { (timeStr, recordStr) ->
    calculateWinPossibilities(timeStr.toLong(), recordStr.toLong())
}.reduce(Long::times)

private fun calculateWinPossibilities(time: Long, record: Long): Long {
    val timeFrame = (1 until time)

    val firstWin = timeFrame
        .first { accelerationTime -> (time - accelerationTime) * accelerationTime > record }

    val lastWin = timeFrame
        .last { accelerationTime -> (time - accelerationTime) * accelerationTime > record }

    return (lastWin - firstWin) + 1
}
