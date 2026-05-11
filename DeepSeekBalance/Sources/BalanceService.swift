import Foundation

// MARK: - API response models

struct BalanceInfo: Codable {
    let currency: String
    let totalBalance: String
    let grantedBalance: String
    let toppedUpBalance: String

    enum CodingKeys: String, CodingKey {
        case currency
        case totalBalance = "total_balance"
        case grantedBalance = "granted_balance"
        case toppedUpBalance = "topped_up_balance"
    }
}

struct BalanceResponse: Codable {
    let isAvailable: Bool
    let balanceInfos: [BalanceInfo]

    enum CodingKeys: String, CodingKey {
        case isAvailable = "is_available"
        case balanceInfos = "balance_infos"
    }
}

// MARK: - Balance service

final class BalanceService {
    private let apiKey: String
    private let session: URLSession
    private var timer: Timer?

    var onUpdate: ((Double, String) -> Void)?
    var onError: ((String) -> Void)?

    init(apiKey: String) {
        self.apiKey = apiKey
        let config = URLSessionConfiguration.ephemeral
        config.timeoutIntervalForRequest = 10
        config.timeoutIntervalForResource = 15
        config.httpMaximumConnectionsPerHost = 1
        config.waitsForConnectivity = false
        self.session = URLSession(configuration: config)
    }

    func start(interval: TimeInterval) {
        fetch()
        timer?.invalidate()
        timer = Timer.scheduledTimer(withTimeInterval: interval, repeats: true) { [weak self] _ in
            self?.fetch()
        }
    }

    func stop() {
        timer?.invalidate()
        timer = nil
    }

    func fetch() {
        guard let url = URL(string: "https://api.deepseek.com/user/balance") else {
            onError?("Invalid URL")
            return
        }

        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        request.setValue("Bearer \(apiKey)", forHTTPHeaderField: "Authorization")
        request.cachePolicy = .reloadIgnoringLocalCacheData

        session.dataTask(with: request) { [weak self] data, response, error in
            if let error = error {
                DispatchQueue.main.async { self?.onError?(error.localizedDescription) }
                return
            }

            guard let data = data,
                  let httpResponse = response as? HTTPURLResponse
            else {
                DispatchQueue.main.async { self?.onError?("No response") }
                return
            }

            guard httpResponse.statusCode == 200 else {
                DispatchQueue.main.async { self?.onError?("HTTP \(httpResponse.statusCode)") }
                return
            }

            do {
                let balance = try JSONDecoder().decode(BalanceResponse.self, from: data)
                guard !balance.balanceInfos.isEmpty else {
                    DispatchQueue.main.async { self?.onError?("Empty balance") }
                    return
                }

                // Prefer CNY, fall back to first available currency
                let info = balance.balanceInfos.first { $0.currency == "CNY" }
                    ?? balance.balanceInfos[0]

                guard let total = Double(info.totalBalance) else {
                    DispatchQueue.main.async { self?.onError?("Bad balance value") }
                    return
                }

                DispatchQueue.main.async { self?.onUpdate?(total, info.currency) }
            } catch {
                DispatchQueue.main.async { self?.onError?("Parse error") }
            }
        }.resume()
    }
}
