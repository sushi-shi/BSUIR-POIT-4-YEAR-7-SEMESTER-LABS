----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:32:33 11/19/2021 
-- Design Name: 
-- Module Name:    D_LATCH_ENABLE_BHVR - Behavioral 
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

entity D_LATCH_ENABLE_BHVR is
    Port ( D : in  STD_LOGIC;
			  E : in  STD_LOGIC;
           Q : out  STD_LOGIC;
           nQ : out  STD_LOGIC);
end D_LATCH_ENABLE_BHVR;

architecture Behavioral of D_LATCH_ENABLE_BHVR is

	signal Q_tmp, nQ_tmp : STD_LOGIC;
	signal S, R : STD_LOGIC;
begin
	S <= E and D;
	R <= E and not D;
	
	Q_tmp <= nQ_tmp nor R;
	nQ_tmp <= Q_tmp nor S;

	Q <= Q_tmp;
	nQ <= nQ_tmp;
end Behavioral;

