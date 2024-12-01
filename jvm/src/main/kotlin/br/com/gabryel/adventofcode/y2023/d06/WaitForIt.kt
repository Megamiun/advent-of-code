package br.com.gabryel.adventofcode.y2023.d06

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

private fun List<Pair<String, String>>.getKerningMistakePossibilities() =
    reduce { (timeAcc, recordAcc), (timeSuffix, recordSuffix) -> (timeAcc + timeSuffix) to (recordAcc + recordSuffix) }
    .let { (time, record) -> calculateWinPossibilities(time.toLong(), record.toLong()) }


private fun List<Pair<String, String>>.getProductOfPossibilities() = map { (timeStr, recordStr) ->
    calculateWinPossibilities(timeStr.toLong(), recordStr.toLong())
}.reduce(Long::times)

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
