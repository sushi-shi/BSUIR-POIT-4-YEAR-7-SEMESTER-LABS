----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    18:24:08 11/16/2021 
-- Design Name: 
-- Module Name:    LUT_5 - Behavioral 
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
use IEEE.STD_LOGIC_ARITH.ALL;
use IEEE.STD_LOGIC_UNSIGNED.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity LUT_5 is
	GENERIC (
		INIT: STD_LOGIC_VECTOR (31 downto 0) := x"000000FF"
	);

    Port ( addr : in  STD_LOGIC_VECTOR (4 downto 0);
           Q : out  STD_LOGIC);
end LUT_5;

architecture Behavioral of LUT_5 is
begin
	Q <=  INIT(CONV_INTEGER(addr));
end Behavioral;