----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:58:13 11/19/2021 
-- Design Name: 
-- Module Name:    DFF_MASTER_SLAVE_V2 - Behavioral 
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

entity DFF_MASTER_SLAVE_V2 is
    Port ( D : in  STD_LOGIC;
           CLK : in  STD_LOGIC;
           Q : out  STD_LOGIC);
end DFF_MASTER_SLAVE_V2;

architecture Behavioral of DFF_MASTER_SLAVE_V2 is
component D_LATCH_ENABLE_BHVR 
PORT(
	D : in STD_LOGIC;
	E : in STD_LOGIC;
	Q : out STD_LOGIC;
	nQ   : out STD_LOGIC);
END component;

component MY_NOT port (
	A: in STD_LOGIC;
	Q: out STD_LOGIC
);
end component;

signal nCLK : STD_LOGIC;
signal  S, R: STD_LOGIC;
signal no, nono: STD_LOGIC;

begin
	U1 : MY_NOT port map (CLK, nCLK);
	
	U2 : D_LATCH_ENABLE_BHVR port map(D, nCLK, S, no);
	U3 : D_LATCH_ENABLE_BHVR port map(S, CLK, R, nono);
	
	Q <= R;

end Behavioral;

