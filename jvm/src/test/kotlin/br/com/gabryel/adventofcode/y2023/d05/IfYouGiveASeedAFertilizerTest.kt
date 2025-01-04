package br.com.gabryel.adventofcode.y2023.d05

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class IfYouGiveASeedAFertilizerTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 5, it, keepBlanks = true) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,35", "input,309796150"])
    fun `Part 1`(file: String, expected: Long) {
        val result = timed("Part 1 - $file") { generateSourceToDestination(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,46", "input,50716416"])
    fun `Part 2`(file: String, expected: Long) {
        val result = timed("Part 2 - $file") { generateSourceToDestinationRanged(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}