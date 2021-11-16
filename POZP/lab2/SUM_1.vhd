----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:14:27 11/16/2021 
-- Design Name: 
-- Module Name:    SUM_1 - Behavioral 
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

entity SUM_1 is
    Port ( X : in  STD_LOGIC;
           Y : in  STD_LOGIC;
           S : out  STD_LOGIC;
           P : out  STD_LOGIC);
end SUM_1;

architecture Behavioral of SUM_1 is

begin
	S <= X xor Y;
	P <= X and Y;
end Behavioral;

