local dap = require('dap')
dap.configurations.rust = {
	{
		name = "Launch day 22",
		type = "codelldb",
		request = "launch",
		program = "${workspaceFolder}/target/debug/day-22.exe",
		stopOnEntry = false,
	},
	{
		name = "Launch day 23",
		type = "codelldb",
		request = "launch",
		program = "${workspaceFolder}/target/debug/day-23.exe",
		stopOnEntry = false,
	},
}
