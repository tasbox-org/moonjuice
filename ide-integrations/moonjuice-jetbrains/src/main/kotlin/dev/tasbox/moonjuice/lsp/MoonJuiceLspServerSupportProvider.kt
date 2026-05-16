package dev.tasbox.moonjuice.lsp

import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.LspServer
import com.intellij.platform.lsp.api.LspServerSupportProvider
import com.intellij.platform.lsp.api.lsWidget.LspServerWidgetItem
import dev.tasbox.moonjuice.MoonJuiceIcons
import dev.tasbox.moonjuice.settings.MoonJuiceSettingsConfigurable

class MoonJuiceLspServerSupportProvider : LspServerSupportProvider {
  override fun fileOpened(
    project: Project,
    file: VirtualFile,
    serverStarter: LspServerSupportProvider.LspServerStarter
  ) {
    if (MoonJuiceLspServerDescriptor.isSupportedFile(file)) {
      serverStarter.ensureServerStarted(MoonJuiceLspServerDescriptor(project))
    }
  }

  override fun createLspServerWidgetItem(lspServer: LspServer, currentFile: VirtualFile?): LspServerWidgetItem =
    LspServerWidgetItem(lspServer, currentFile, MoonJuiceIcons.Language, MoonJuiceSettingsConfigurable::class.java)
}
