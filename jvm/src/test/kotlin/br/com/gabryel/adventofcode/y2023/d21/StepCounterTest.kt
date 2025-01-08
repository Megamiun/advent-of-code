package br.com.gabryel.adventofcode.y2023.d21

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class StepCounterTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 21, it) }

    @ParameterizedTest(name = "Part 1 - {0} - {1} steps")
    @CsvSource(value = ["sample,6,16", "input,64,3594"])
    fun `Part 1`(file: String, steps: Int, expected: Long) {
        val result = timed("Part 1 - $file - $steps steps") { countStepsFor(files[file]!!, steps) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0} - {1} steps")
    @CsvSource(value = ["sample,50,1594", "sample,500,167004", "sample,5000,16733044", "input,26501365,605247138198755"])
    fun `Part 2`(file: String, steps: Int, expected: Long) {
        val result = timed("Part 2 - $file - $steps steps") { countStepsForInfinite(files[file]!!, steps) }

        assertThat(result, equalTo(expected))
    }
}