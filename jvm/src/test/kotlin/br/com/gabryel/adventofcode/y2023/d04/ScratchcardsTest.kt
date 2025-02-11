package br.com.gabryel.adventofcode.y2023.d04

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class ScratchcardsTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 4, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,13", "input,20407"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { sumCardsValues(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,30", "input,23806951"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { sumCardsRecursive(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}