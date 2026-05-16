package dev.tasbox.moonjuice.settings

import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory
import com.intellij.openapi.options.BoundSearchableConfigurable
import com.intellij.openapi.ui.DialogPanel
import com.intellij.ui.dsl.builder.bindItem
import com.intellij.ui.dsl.builder.panel
import dev.tasbox.moonjuice.MoonJuiceBundle

internal class MoonJuiceSettingsConfigurable : BoundSearchableConfigurable(
  "MoonJuice",
  "MoonJuice"
) {
  private val settings
    get() = MoonJuiceSettings.getInstance();

  override fun createPanel(): DialogPanel {
    return panel {
      row(MoonJuiceBundle.message("moonjuice.settings.lsp.path.label")) {
        comboBoxWithBrowseButton(
          fileChooserDescriptor = FileChooserDescriptorFactory.createSingleFolderDescriptor()
            .withTitle(MoonJuiceBundle.message("moonjuice.settings.lsp.path.dialog.title"))
            .withFileFilter { file -> file.nameWithoutExtension == "moonjuice-lsp" },
        ).bindItem(settings::lspPath, { settings.lspPath = it ?: FROM_PATH_META_OPTION })
      }
    }
  }
}
