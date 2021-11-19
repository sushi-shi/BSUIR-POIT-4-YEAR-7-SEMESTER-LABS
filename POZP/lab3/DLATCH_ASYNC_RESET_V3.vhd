----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:26:36 11/19/2021 
-- Design Name: 
-- Module Name:    DLATCH_ASYNC_RESET_V3 - Behavioral 
-- Project Name: 
-- Target Devices: 
-- Tool versions: 
-- Description: 
--
-- Dependencies: 
--
-- Revision: 
-- Revision 0.01 - File Created
-- Additional Comments: 
--
----------------------------------------------------------------------------------
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity DLATCH_ASYNC_RESET is
    Port ( D : in  STD_LOGIC;
			  E : in  STD_LOGIC;
			  CLR : in STD_LOGIC;
           Q : out  STD_LOGIC;
           nQ : out  STD_LOGIC);
end DLATCH_ASYNC_RESET;

architecture Behavioral of DLATCH_ASYNC_RESET is
	 signal R, S: std_logic;
    signal QBuf : std_logic;
    signal nQBuf : std_logic;
begin
    QBuf <= not (((E and not D) or CLR) or nQBuf);
    nQBuf <= not (((E and D) xor CLR) or QBuf);
	 
    Q <= QBuf;
    nQ <= nQBuf;
end Behavioral;