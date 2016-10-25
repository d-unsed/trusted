require 'spec_helper'

describe Trusted::Config::Builder do
  let(:binding_type) { :tcp }
  let(:address) { '127.0.0.1:8080' }
  let(:config) { builder.config }

  subject(:builder) { described_class.new }

  describe '.dsl' do
    let(:dsl_block) { -> {} }

    subject { described_class.dsl(&dsl_block) }

    it 'uses docile for handling DSL' do
      builder = instance_double(described_class)

      allow(described_class).to receive(:new).and_return(builder)

      allow(Docile).to receive(:dsl_eval) do |arg_builder, &block|
        expect(arg_builder).to eq(builder)
        expect(block).to eq(dsl_block)

        builder
      end

      expect(builder).to receive(:build)

      subject
    end
  end

  describe '#binding_type' do
    subject { builder.binding_type(binding_type) }

    it 'sets correct binding type' do
      expect { subject }.to change { config[:binding_type] }.to(binding_type)
    end
  end

  describe '#listen_on' do
    subject { builder.listen_on(address) }

    it 'sets correct address' do
      expect { subject }.to change { config[:listen_on] }.to(address)
    end
  end

  describe '#build' do
    subject { builder.build }

    context 'with default configuration' do
      it 'creates a new instance of Config with default values' do
        expect(Trusted::Config::Config)
          .to receive(:new)
          .with(described_class::DEFAULT_CONFIG)

        subject
      end
    end

    context 'with custom configuration' do
      before do
        builder.binding_type(binding_type)
        builder.listen_on(address)
      end

      it 'creates a new instance of Config' do
        expect(Trusted::Config::Config)
          .to receive(:new)
          .with(
            binding_type: binding_type,
            listen_on: address,
          )

          subject
      end
    end
  end
end
