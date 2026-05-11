import Cocoa

final class AppDelegate: NSObject, NSApplicationDelegate {
    private var menuBarController: MenuBarController!
    private var balanceService: BalanceService?
    private var settingsWindow: SettingsWindowController?
    private var currentApiKey: String?
    private var refreshInterval: TimeInterval = 300 // 5 min default

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSApp.setActivationPolicy(.accessory)

        menuBarController = MenuBarController()
        menuBarController.setup()

        menuBarController.onRefreshTapped = { [weak self] in
            self?.balanceService?.fetch()
        }
        menuBarController.onSettingsTapped = { [weak self] in
            self?.openSettings()
        }
        menuBarController.onQuitTapped = {
            NSApp.terminate(nil)
        }

        // Load key and start
        if let key = KeychainService.load(), !key.isEmpty {
            currentApiKey = key
            loadRefreshInterval()
            startBalanceService()
        } else {
            menuBarController.showMissingKey()
            // Delay slightly so UI is ready
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
                self?.openSettings()
            }
        }
    }

    func applicationWillTerminate(_ notification: Notification) {
        balanceService?.stop()
    }

    // MARK: - Balance service

    private func startBalanceService() {
        guard let key = currentApiKey, !key.isEmpty else { return }

        balanceService?.stop()
        balanceService = BalanceService(apiKey: key)
        balanceService?.onUpdate = { [weak self] amount, currency in
            self?.menuBarController.showBalance(amount, currency: currency)
        }
        balanceService?.onError = { [weak self] message in
            self?.menuBarController.showError(message)
        }
        balanceService?.start(interval: refreshInterval)
        menuBarController.showLoading()
    }

    // MARK: - Settings

    private func openSettings() {
        if settingsWindow == nil {
            settingsWindow = SettingsWindowController()
            settingsWindow?.onSave = { [weak self] key, interval in
                self?.handleSettingsSave(apiKey: key, interval: interval)
            }
        }

        settingsWindow?.populate(
            apiKey: currentApiKey ?? "",
            interval: refreshInterval)
        settingsWindow?.showWindow(nil)
        settingsWindow?.window?.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
    }

    private func handleSettingsSave(apiKey: String, interval: TimeInterval) {
        guard KeychainService.save(apiKey: apiKey) else {
            let alert = NSAlert()
            alert.messageText = "Keychain Error"
            alert.informativeText = "Failed to save API key to Keychain."
            alert.runModal()
            return
        }

        currentApiKey = apiKey
        refreshInterval = interval
        UserDefaults.standard.set(interval, forKey: "refreshInterval")
        startBalanceService()
    }

    private func loadRefreshInterval() {
        let stored = UserDefaults.standard.double(forKey: "refreshInterval")
        if stored >= 60 { // at least 1 minute
            refreshInterval = stored
        }
    }
}
