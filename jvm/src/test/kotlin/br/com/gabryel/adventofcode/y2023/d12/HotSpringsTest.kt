package br.com.gabryel.adventofcode.y2023.d12

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class HotSpringsTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 12, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,21", "input,8193"])
    fun `Part 1`(file: String, expected: Long) {
        val result = timed("Part 1 - $file") { findHotSpringsPossibilities(files[file]!!, 1) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,525152", "input,45322533163795"])
    fun `Part 2`(file: String, expected: Long) {
        val result = timed("Part 2 - $file") { findHotSpringsPossibilities(files[file]!!, 5) }

        assertThat(result, equalTo(expected))
    }
}