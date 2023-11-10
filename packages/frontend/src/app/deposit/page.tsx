'use client'

import { Connect } from '../../components/Connect'
import { Connected } from '../../components/Connected'
import { RegisterUnclaim } from '../../components/RegisterUnclaim'
import { useEffect, useRef, useState } from "react";
import { useDebounce } from "use-debounce";

export default function Page() {
  const [email, setEmail] = useState<string>("");
  const [amount, setAmount] = useState<string>("0");
  const [currency, setCurrency] = useState<string>("ETH");
  const [dropdownOpen, setDropdownOpen] = useState<boolean>(false);
  const dropdownRef = useRef(null);
  const [debouncedEmail] = useDebounce(email, 500)
  const [debouncedAmount] = useDebounce(amount, 500)

  return (
    <div>
      <div className="flex flex-col text-center py-8 sm:py-24">
        <div className="max-w-3xl mx-auto p-4">
          <div className="flex flex-col gap-4">
            <div className="rounded-lg w-full p-2.5 bg-slate-200 flex justify-between">
              <input
                type="string"
                placeholder="Amount to send"
                onChange={(e) => {
                  setAmount(e.target.value);
                }}
                onBlur={(e) => {
                  setAmount(e.target.value);
                }}
                className="text-sm bg-slate-200 focus:outline-none"
                value={amount || ""}
              />

              <div
                className="relative inline-block text-left"
                ref={dropdownRef}
              >
                <div>
                  <button
                    type="button"
                    className="inline-flex w-full justify-center gap-x-1.5 rounded-lg bg-slate-800 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-slate-700"
                    id="menu-button"
                    aria-expanded="true"
                    aria-haspopup="true"
                    onClick={() => setDropdownOpen(!dropdownOpen)}
                  >
                    {currency}
                    <svg
                      className="-mr-1 h-5 w-5 text-gray-400"
                      viewBox="0 0 20 20"
                      fill="currentColor"
                      aria-hidden="true"
                    >
                      <path
                        fillRule="evenodd"
                        d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                        clipRule="evenodd"
                      />
                    </svg>
                  </button>
                </div>
                {dropdownOpen && (
                  <div
                    className="absolute right-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                    role="menu"
                    aria-orientation="vertical"
                    aria-labelledby="menu-button"
                    tabIndex={-1}
                  >
                    <div className="py-1" role="none">
                      <span
                        className={getCurrencyOptionClass(
                          currency === "ETH",
                        )}
                        role="menuitem"
                        onClick={() => {
                          setCurrency("ETH");
                          setDropdownOpen(false);
                        }}
                      >
                        ETH
                      </span>
                      <span
                        className={getCurrencyOptionClass(
                          currency === "DAI",
                        )}
                        role="menuitem"
                        onClick={() => {
                          setCurrency("DAI");
                          setDropdownOpen(false);
                        }}
                      >
                        DAI
                      </span>
                      <span
                        className={getCurrencyOptionClass(
                          currency === "USDC",
                        )}
                        role="menuitem"
                        onClick={() => {
                          setCurrency("USDC");
                          setDropdownOpen(false);
                        }}
                      >
                        USDC
                      </span>
                    </div>
                  </div>
                )}
              </div>
            </div>

            <input
              type="email"
              className="text-sm rounded-lg block w-full p-2.5 
                bg-slate-200 focus:outline-none focus:border-sky-500 focus:ring-1 focus:ring-sky-500
                disabled:bg-slate-50 disabled:text-slate-500 disabled:border-slate-200 disabled:shadow-none
                invalid:border-pink-500 invalid:text-pink-600
                focus:invalid:border-pink-500 focus:invalid:ring-pink-500"
              placeholder="recipient@email.address"
              onChange={(e) => {
                setEmail(e.target.value);
              }}
              onBlur={(e) => {
                setEmail(e.target.value);
              }}
            />
            <Connect />

            <Connected>
              <p>Connected</p>
              <RegisterUnclaim toEmailAddr={debouncedEmail} tokenName={currency} amountStr={debouncedAmount}></RegisterUnclaim>
            </Connected>
          </div>
        </div>
      </div>
    </div >
  )
}

function isValidEmail(email: string): boolean {
  const regex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,16}$/;
  return regex.test(email);
}

function getCurrencyOptionClass(selected: boolean): string {
  const baseClasses =
    "text-gray-700 block px-4 py-2 text-sm m-2 rounded-md cursor-pointer hover:transition-all";
  return selected
    ? `${baseClasses} bg-slate-100`
    : `${baseClasses} hover:bg-slate-100`;
}