require 'spec_helper'

describe Trusted::Config::Config do
  subject(:config) { described_class.new(configuration) }

  describe '#binding_type' do
    let(:binding_type) { :tcp }

    let(:configuration) do
      {
        binding_type: binding_type,
      }
    end

    its(:binding_type) { is_expected.to eq(binding_type) }
  end

  describe '#listen_on' do
    let(:address) { 'localhost:8080' }

    let(:configuration) do
      {
        listen_on: address,
      }
    end

    its(:listen_on) { is_expected.to eq(address) }
  end
end
