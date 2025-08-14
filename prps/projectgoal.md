
# Project plan 

## AI Directive
1. If project plan already exist, modify and update as work progress.
2. Modify and update tasks.md as needed as project progess.

## Goal
- Build an app that connects to interactive broker trader workstation (equTWS)
- Utilize the rust crate `https://github.com/wboayue/rust-ibapi`
- This app will be a terminal app that perform the following steps:
  1. Connect to Trader Workstation on port 7500
  2. If the asset (i.e. stock) is not pass as an argument, ask for the ticker symbol
  3. Check the current position of an asset (i.e. a stock) such as Apple Stock
  4. There should be no position of the current asset.  If position exist, warn user.
  5. Retrieve the current real time price through TWS
  6. Prompt user with current price and ask if user wish to enter long position.
  7. If and when position is opened, track portfolio value for the current asset
  8. When portfolio value has been updated, ask trader to sell position.
  9. Sell the position if trader has approved the sale.

## UI Requirement
- This is a terminal app.
- Clear the terminal when starting up.
- Refresh terminal and display Profit and Loss (PnL) for an asset.
- Display loss in red and profit in green
- PnL should be visible and present at the top like this `*** PnL: $80.00`
- Display current price on 2nd line ` *** AAPL : $160. 00` in orange
- Third line prompt user ` >> Buy (y/n) ?` or ` >> Sell (y/n) ` depending on position status
- When position is closed, display final PnL.

