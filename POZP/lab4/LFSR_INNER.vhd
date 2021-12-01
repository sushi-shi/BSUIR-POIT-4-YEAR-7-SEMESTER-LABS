----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    13:34:52 11/20/2021 
-- Design Name: 
-- Module Name:    LFSR_INNER - Behavioral 
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

entity LFSR_INNER is
    generic (
	 -- f(x) = 1 xor x xor x^6;
		alpha : std_logic_vector := "1100001"
	 );
    port (
		CLK : in std_logic;
		RST : in std_logic;
		Pout: out std_logic_vector(0 to alpha'high - 1)
	);
end LFSR_INNER;

architecture Behavioral of LFSR_INNER is

signal sreg: std_logic_vector(0 to alpha'high - 1);
signal sdat: std_logic_vector(0 to alpha'high - 1);

begin

  main: process( CLK, RST, sdat )
  begin
    if RST = '1' then
      sreg <= ( others => '0' );
    elsif rising_edge(CLK) then
      sreg <= sdat;
    end if;
  end process;
  
  data: process( sreg ) 
    variable zerostate : std_logic;
  begin
	 zerostate := '0';
	 
	 for i in 0 to alpha'high - 2 loop
		zerostate:= zerostate or sreg(i);
		
		if alpha(i+1) = '1' then
			sdat(i+1) <= sreg( alpha'high - 1 ) xor sreg(i);
		else 
			sdat(i+1) <= sreg( i );
		end if;
	 end loop;
	 -- equals to 1 when everything is 0
	 zerostate := not zerostate;
	 
	 if zerostate = '1' then
		sdat(0) <= '1';
	 else 
		sdat(0) <= sreg( alpha'high - 1 );
	 end if;
  end process;

  Pout <= sdat;
    
end Behavioral;
