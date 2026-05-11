import Cocoa

final class MenuBarController: NSObject {
    private var statusItem: NSStatusItem!
    private var menu: NSMenu!
    private var balanceHeaderItem: NSMenuItem!

    var onRefreshTapped: (() -> Void)?
    var onSettingsTapped: (() -> Void)?
    var onQuitTapped: (() -> Void)?

    func setup() {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        if let button = statusItem.button {
            button.title = "..."
            button.font = NSFont.monospacedDigitSystemFont(
                ofSize: NSFont.smallSystemFontSize, weight: .regular)
        }

        menu = NSMenu()
        menu.minimumWidth = 180

        balanceHeaderItem = NSMenuItem(title: "Loading...", action: nil, keyEquivalent: "")
        balanceHeaderItem.isEnabled = false
        menu.addItem(balanceHeaderItem)

        menu.addItem(.separator())

        let refreshItem = NSMenuItem(
            title: "Refresh Now", action: #selector(refreshAction), keyEquivalent: "r")
        refreshItem.target = self
        menu.addItem(refreshItem)

        let settingsItem = NSMenuItem(
            title: "Settings...", action: #selector(settingsAction), keyEquivalent: ",")
        settingsItem.target = self
        menu.addItem(settingsItem)

        menu.addItem(.separator())

        let quitItem = NSMenuItem(
            title: "Quit", action: #selector(quitAction), keyEquivalent: "q")
        quitItem.target = self
        menu.addItem(quitItem)

        statusItem.menu = menu
    }

    // MARK: - Public update methods

    func showBalance(_ amount: Double, currency: String) {
        let symbol = currency == "USD" ? "$" : "¥"
        statusItem.button?.title = String(format: "%@%.2f", symbol, amount)
        updateHeader(currency: currency, total: amount)
    }

    func showError(_ message: String) {
        statusItem.button?.title = "⚠️"
        balanceHeaderItem.title = "Error: \(message)"
    }

    func showLoading() {
        statusItem.button?.title = "..."
        balanceHeaderItem.title = "Loading..."
    }

    func showMissingKey() {
        statusItem.button?.title = "🔑"
        balanceHeaderItem.title = "API Key not set — open Settings"
    }

    // MARK: - Private

    private func updateHeader(currency: String, total: Double) {
        let symbol = currency == "USD" ? "$" : "¥"
        balanceHeaderItem.title = "Balance: \(symbol)\(String(format: "%.2f", total))"
    }

    @objc private func refreshAction() {
        showLoading()
        onRefreshTapped?()
    }

    @objc private func settingsAction() {
        onSettingsTapped?()
    }

    @objc private func quitAction() {
        onQuitTapped?()
    }
}
