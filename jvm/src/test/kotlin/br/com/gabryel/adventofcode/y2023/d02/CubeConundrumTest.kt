package br.com.gabryel.adventofcode.y2023.d02

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class CubeConundrumTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 2, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,8", "input,2256"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getSumOfValidCubeConumdrumGames(files[file]!!, listOf(12, 13, 14)) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,2286", "input,74229"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getMinimumSetOfCubesForCubeConundrum(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}