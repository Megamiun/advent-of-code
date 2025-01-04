package br.com.gabryel.adventofcode.y2023.d07

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class CamelCardsTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 7, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,6440", "input,248453531"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { findSimple(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,5905", "input,248781813"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { findJokers(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}