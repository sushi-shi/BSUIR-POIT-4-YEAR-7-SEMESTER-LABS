----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:00:38 11/19/2021 
-- Design Name: 
-- Module Name:    RS_LATCH_BHVR - Behavioral 
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

entity RS_LATCH_BHVR is
    Port ( S : in  STD_LOGIC;
           R : in  STD_LOGIC;
           Q : inout  STD_LOGIC;
           nQ : inout  STD_LOGIC);
end RS_LATCH_BHVR;

architecture Behavioral of RS_LATCH_BHVR is

begin
	Q <= nQ nor R;
	nQ <=  Q nor S;
end Behavioral;

