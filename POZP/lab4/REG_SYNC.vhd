----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    01:27:28 11/20/2021 
-- Design Name: 
-- Module Name:    REG_SYNC - Behavioral 
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

entity REG_SYNC is
	generic (N : integer := 8);
	port (
		DIN : IN STD_LOGIC_VECTOR(N - 1 downto 0);
		E   : IN STD_LOGIC;
		CLK : IN STD_LOGIC;
		OE  : IN STD_LOGIC;
		DOUT: OUT STD_LOGIC_VECTOR(N - 1 downto 0));
end REG_SYNC;

architecture Behavioral of REG_SYNC is
	 
signal reg : STD_LOGIC_VECTOR(N-1 downto 0);
constant ALLZ : STD_LOGIC_VECTOR(N-1 downto 0) := (others => 'Z');

begin
	main : process(DIN, E, CLK)
	begin
		if E = '1' and rising_edge(CLK) then 
			reg <= DIN;
		end if;
	end process;
	
	DOUT <= reg when OE = '1' else ALLZ;
end Behavioral;

