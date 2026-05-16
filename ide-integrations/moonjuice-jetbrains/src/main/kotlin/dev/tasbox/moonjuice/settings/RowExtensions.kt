package dev.tasbox.moonjuice.settings

import com.intellij.openapi.fileChooser.FileChooser
import com.intellij.openapi.fileChooser.FileChooserDescriptor
import com.intellij.openapi.ui.ComboBox
import com.intellij.ui.dsl.builder.Cell
import com.intellij.ui.dsl.builder.Row

// TODO: Properly display the various options without hardcoding strings
fun Row.comboBoxWithBrowseButton(
  fileChooserDescriptor: FileChooserDescriptor,
): Cell<ComboBox<String>> {
  val comboBox = ComboBox(arrayOf(FROM_PATH_META_OPTION)).apply {
    @Suppress("UnstableApiUsage") initBrowsableEditor({
      FileChooser.chooseFile(fileChooserDescriptor, null, null) {
        this.removeAllItems()

        this.addItem(FROM_PATH_META_OPTION)
        this.addItem(it.path)

        this.selectedItem = it.path
      }
    }, null)
  }

  val result = cell(comboBox).applyToComponent {
    isOpaque = false
  }

  return result
}
