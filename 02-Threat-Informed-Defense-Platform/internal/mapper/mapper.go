package mapper

import "strings"

func NameForTechnique(id string) string {
	names := map[string]string{
		"T1059": "Command and Scripting Interpreter",
		"T1078": "Valid Accounts",
		"T1047": "Windows Management Instrumentation",
		"T1003": "OS Credential Dumping",
		"T1105": "Ingress Tool Transfer",
	}

	if name, ok := names[strings.ToUpper(id)]; ok {
		return name
	}

	return "Unknown / unmapped ATT&CK technique"
}

func TacticForTechnique(id string) string {
	tactics := map[string]string{
		"T1059": "Execution",
		"T1078": "Defense Evasion / Persistence / Initial Access",
		"T1047": "Execution",
		"T1003": "Credential Access",
		"T1105": "Command and Control",
	}

	if tactic, ok := tactics[strings.ToUpper(id)]; ok {
		return tactic
	}

	return "Unmapped"
}

func TelemetryForTechnique(id string) []string {
	switch strings.ToUpper(id) {
	case "T1059":
		return []string{"process_creation", "command_line", "script_block_logging", "parent_child_process"}
	case "T1078":
		return []string{"authentication_logs", "account_logon", "privilege_changes", "unusual_login_patterns"}
	case "T1047":
		return []string{"process_creation", "wmi_activity", "event_logs", "remote_execution_indicators"}
	case "T1003":
		return []string{"process_access", "sensitive_file_access", "credential_store_access", "endpoint_alerts"}
	case "T1105":
		return []string{"network_connections", "file_creation", "dns_logs", "proxy_logs"}
	default:
		return []string{"process_creation", "authentication_logs", "network_connections"}
	}
}

func PriorityForConfidence(confidence string) string {
	switch strings.ToLower(strings.TrimSpace(confidence)) {
	case "high":
		return "high"
	case "medium":
		return "medium"
	case "low":
		return "low"
	default:
		return "review"
	}
}
