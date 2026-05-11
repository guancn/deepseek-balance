import Cocoa

// MARK: - Custom window that restores clipboard shortcuts

/// NSWindow subclass that intercepts ⌘V / ⌘C / ⌘X / ⌘A and forwards them
/// to the first responder.  Needed because LSUIElement apps have no Edit menu,
/// so the standard menu-based shortcut dispatch never fires.
final class SettingsWindow: NSWindow {
    override func performKeyEquivalent(with event: NSEvent) -> Bool {
        // Give standard dispatch (controls, etc.) a chance first.
        if super.performKeyEquivalent(with: event) {
            return true
        }

        guard event.type == .keyDown,
              event.modifierFlags.intersection(.deviceIndependentFlagsMask) == .command,
              let chars = event.charactersIgnoringModifiers
        else { return false }

        let sel: Selector
        switch chars {
        case "v": sel = #selector(NSText.paste(_:))
        case "c": sel = #selector(NSText.copy(_:))
        case "x": sel = #selector(NSText.cut(_:))
        case "a": sel = #selector(NSText.selectAll(_:))
        default:  return false
        }

        guard let fr = firstResponder else { return false }

        // Walk up the responder chain until we find one that handles the selector.
        var responder: NSResponder? = fr
        while let r = responder {
            if r.responds(to: sel) {
                r.perform(sel, with: nil)
                return true
            }
            responder = r.nextResponder
        }

        return false
    }
}

// MARK: - Settings window controller

final class SettingsWindowController: NSWindowController {
    private var secureField: NSTextField!
    private var plainField: NSTextField!
    private var showCheckbox: NSButton!
    private var intervalPopup: NSPopUpButton!

    var onSave: ((String, TimeInterval) -> Void)?

    convenience init() {
        let window = SettingsWindow(
            contentRect: NSRect(x: 0, y: 0, width: 380, height: 210),
            styleMask: [.titled, .closable],
            backing: .buffered,
            defer: false)
        window.title = "Settings"
        window.center()
        window.isReleasedWhenClosed = false
        self.init(window: window)
        buildUI()
    }

    func populate(apiKey: String, interval: TimeInterval) {
        secureField.stringValue = apiKey
        plainField.stringValue = apiKey
        showCheckbox.state = .off
        toggleFields(show: false)

        let minutes = Int(interval / 60)
        intervalPopup.selectItem(withTag: minutes)
    }

    // MARK: - UI

    private func buildUI() {
        guard let content = window?.contentView else { return }

        // — API Key label —
        let keyLabel = NSTextField(labelWithString: "DeepSeek API Key:")
        keyLabel.frame = NSRect(x: 20, y: 175, width: 340, height: 16)
        content.addSubview(keyLabel)

        // — Secure field —
        secureField = NSSecureTextField(frame: NSRect(x: 20, y: 148, width: 340, height: 22))
        secureField.placeholderString = "sk-..."
        content.addSubview(secureField)

        // — Plain field (hidden by default, same position) —
        plainField = NSTextField(frame: NSRect(x: 20, y: 148, width: 340, height: 22))
        plainField.placeholderString = "sk-..."
        plainField.isHidden = true
        plainField.font = secureField.font
        content.addSubview(plainField)

        // — Show checkbox —
        showCheckbox = NSButton(
            checkboxWithTitle: "Show API Key", target: self, action: #selector(toggleShow))
        showCheckbox.frame = NSRect(x: 20, y: 120, width: 140, height: 20)
        showCheckbox.font = NSFont.systemFont(ofSize: NSFont.smallSystemFontSize)
        content.addSubview(showCheckbox)

        // — Interval label —
        let intervalLabel = NSTextField(labelWithString: "Refresh Interval:")
        intervalLabel.frame = NSRect(x: 20, y: 90, width: 340, height: 16)
        content.addSubview(intervalLabel)

        // — Interval popup —
        intervalPopup = NSPopUpButton(frame: NSRect(x: 20, y: 60, width: 140, height: 22))
        intervalPopup.addItems(withTitles: [
            "1 minute", "5 minutes", "10 minutes", "30 minutes", "1 hour",
        ])
        intervalPopup.item(at: 0)?.tag = 1
        intervalPopup.item(at: 1)?.tag = 5
        intervalPopup.item(at: 2)?.tag = 10
        intervalPopup.item(at: 3)?.tag = 30
        intervalPopup.item(at: 4)?.tag = 60
        intervalPopup.selectItem(at: 1)
        content.addSubview(intervalPopup)

        // — Save button —
        let saveButton = NSButton(title: "Save", target: self, action: #selector(saveAction))
        saveButton.frame = NSRect(x: 280, y: 14, width: 80, height: 24)
        saveButton.bezelStyle = .rounded
        saveButton.keyEquivalent = "\r"
        content.addSubview(saveButton)
    }

    // MARK: - Actions

    @objc private func toggleShow() {
        toggleFields(show: showCheckbox.state == .on)
    }

    private func toggleFields(show: Bool) {
        if show {
            plainField.stringValue = secureField.stringValue
        } else {
            secureField.stringValue = plainField.stringValue
        }
        secureField.isHidden = show
        plainField.isHidden = !show
    }

    @objc private func saveAction() {
        let key = (plainField.isHidden ? secureField.stringValue : plainField.stringValue)
            .trimmingCharacters(in: .whitespacesAndNewlines)

        guard !key.isEmpty else {
            shake(field: plainField.isHidden ? secureField : plainField)
            return
        }

        secureField.stringValue = key
        plainField.stringValue = key

        let minutes = intervalPopup.selectedItem?.tag ?? 5
        onSave?(key, TimeInterval(minutes * 60))
        window?.close()
    }

    private func shake(field: NSView) {
        field.wantsLayer = true
        let animation = CAKeyframeAnimation(keyPath: "position.x")
        animation.values = [0, 8, -8, 4, -4, 0]
        animation.duration = 0.3
        animation.isAdditive = true
        field.layer?.add(animation, forKey: "shake")
    }
}
