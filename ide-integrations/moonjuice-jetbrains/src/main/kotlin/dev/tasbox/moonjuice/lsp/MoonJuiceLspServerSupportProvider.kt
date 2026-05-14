package dev.tasbox.moonjuice.lsp

import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.LspServerSupportProvider

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
}
