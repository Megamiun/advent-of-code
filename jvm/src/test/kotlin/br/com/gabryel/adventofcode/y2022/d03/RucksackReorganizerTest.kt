package br.com.gabryel.adventofcode.y2022.d03

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class RucksackReorganizerTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2022, 3, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,157", "input,7917"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getRucksackRepeatedPriorities(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,70", "input,2585"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getRucksackRepeatedPrioritiesBetweenTrios(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}